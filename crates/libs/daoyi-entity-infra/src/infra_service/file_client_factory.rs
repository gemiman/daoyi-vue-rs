use crate::infra_entity::infra_file_content;
use daoyi_common_support::database;
use daoyi_common_support::enumeration::FileStorageEnum;
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::serde::validate_and_parse;
use daoyi_common_support::vo::infra_vo::{
    DbFileClientConfig, FtpFileClientConfig, LocalFileClientConfig, S3FileClientConfig,
    SftpFileClientConfig,
};
use sea_orm::prelude::Json;
use sea_orm::prelude::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use std::io::Write;
use std::net::TcpStream;
use std::path::Path;
use tokio::io::AsyncWriteExt;

#[async_trait::async_trait]
pub trait FileClient: Send + Sync {
    /// Uploads content to the specified path.
    /// Returns the full URL of the uploaded file.
    async fn upload(&self, content: &[u8], path: &str, content_type: &str) -> ApiResult<String>;

    /// Deletes the file at the specified path.
    async fn delete(&self, path: &str) -> ApiResult<()>;

    /// Returns the content of the file at the specified path.
    async fn get_content(&self, path: &str) -> ApiResult<Vec<u8>>;

    /// Returns a presigned URL for uploading a file to the specified path.
    /// Only supported by S3-compatible storage.
    async fn presign_put_url(&self, _path: &str) -> ApiResult<String> {
        Err(ApiError::biz("当前存储器不支持预签名"))
    }
}

// ================== DB File Client ==================

pub struct DbFileClient {
    config: DbFileClientConfig,
    config_id: String,
}

impl DbFileClient {
    pub fn new(config_id: String, config: DbFileClientConfig) -> Self {
        Self { config, config_id }
    }
}

#[async_trait::async_trait]
impl FileClient for DbFileClient {
    async fn upload(&self, content: &[u8], path: &str, _content_type: &str) -> ApiResult<String> {
        let db = database::get_db_async().await;

        // 删除旧的（如果存在）
        self.delete(path).await?;

        let model = infra_file_content::ActiveModel {
            config_id: Set(self.config_id.clone()),
            path: Set(path.to_string()),
            content: Set(content.to_vec()),
            ..Default::default()
        };
        model.insert(&db).await?;

        // URL logic: domain + path
        Ok(format!("{}/{}", self.config.domain, path))
    }

    async fn delete(&self, path: &str) -> ApiResult<()> {
        let db = database::get_db_async().await;
        infra_file_content::Entity::delete_many()
            .filter(infra_file_content::Column::ConfigId.eq(&self.config_id))
            .filter(infra_file_content::Column::Path.eq(path))
            .exec(&db)
            .await?;
        Ok(())
    }

    async fn get_content(&self, path: &str) -> ApiResult<Vec<u8>> {
        let db = database::get_db_async().await;
        let model = infra_file_content::Entity::find()
            .filter(infra_file_content::Column::ConfigId.eq(&self.config_id))
            .filter(infra_file_content::Column::Path.eq(path))
            .one(&db)
            .await?
            .ok_or_else(|| ApiError::biz("文件不存在"))?;
        Ok(model.content)
    }
}

// ================== Local File Client ==================

pub struct LocalFileClient {
    config: LocalFileClientConfig,
}

impl LocalFileClient {
    pub fn new(config: LocalFileClientConfig) -> Self {
        Self { config }
    }
}

#[async_trait::async_trait]
impl FileClient for LocalFileClient {
    async fn upload(&self, content: &[u8], path: &str, _content_type: &str) -> ApiResult<String> {
        let full_path = Path::new(&self.config.base_path).join(path);

        // Ensure parent directory exists
        if let Some(parent) = full_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| ApiError::biz(format!("创建本地目录失败: {}", e)))?;
        }

        let mut file = tokio::fs::File::create(&full_path)
            .await
            .map_err(|e| ApiError::biz(format!("创建本地文件失败: {}", e)))?;

        file.write_all(content)
            .await
            .map_err(|e| ApiError::biz(format!("写入本地文件失败: {}", e)))?;

        Ok(format!("{}/{}", self.config.domain, path))
    }

    async fn delete(&self, path: &str) -> ApiResult<()> {
        let full_path = Path::new(&self.config.base_path).join(path);
        if full_path.exists() {
            tokio::fs::remove_file(full_path)
                .await
                .map_err(|e| ApiError::biz(format!("删除本地文件失败: {}", e)))?;
        }
        Ok(())
    }

    async fn get_content(&self, path: &str) -> ApiResult<Vec<u8>> {
        let full_path = Path::new(&self.config.base_path).join(path);
        let content = tokio::fs::read(full_path)
            .await
            .map_err(|e| ApiError::biz(format!("读取本地文件失败: {}", e)))?;
        Ok(content)
    }
}

// ================== S3 File Client ==================

pub struct S3FileClient {
    config: S3FileClientConfig,
    client: aws_sdk_s3::Client,
}

impl S3FileClient {
    pub fn new(config: S3FileClientConfig) -> Self {
        let region =
            aws_config::Region::new(config.region.clone().unwrap_or("hangzhou".to_string()));
        let credentials = aws_credential_types::Credentials::new(
            config.access_key.clone(),
            config.access_secret.clone(),
            None,
            None,
            "Static",
        );

        let mut builder = aws_sdk_s3::config::Builder::new()
            .region(region)
            .credentials_provider(credentials)
            .endpoint_url(&config.endpoint)
            .behavior_version(aws_sdk_s3::config::BehaviorVersion::latest());

        if config.enable_path_style_access {
            builder = builder.force_path_style(true);
        }

        let aws_config = builder.build();
        let client = aws_sdk_s3::Client::from_conf(aws_config);

        Self { config, client }
    }
}

#[async_trait::async_trait]
impl FileClient for S3FileClient {
    async fn upload(&self, content: &[u8], path: &str, content_type: &str) -> ApiResult<String> {
        self.client
            .put_object()
            .bucket(&self.config.bucket)
            .key(path)
            .body(aws_sdk_s3::primitives::ByteStream::from(content.to_vec()))
            .content_type(content_type)
            .send()
            .await
            .map_err(|e| {
                let error_msg = format!("S3 上传失败 (Endpoint: {})", self.config.endpoint);
                // Try to log it if tracing is available, otherwise just return it
                tracing::error!("{error_msg}: {e:#?}");
                ApiError::biz(error_msg)
            })?;

        let domain = self
            .config
            .domain
            .as_deref()
            .unwrap_or(&self.config.endpoint);
        // Clean up domain if it ends with /
        let domain = domain.trim_end_matches('/');
        Ok(format!("{}/{}", domain, path))
    }

    async fn delete(&self, path: &str) -> ApiResult<()> {
        self.client
            .delete_object()
            .bucket(&self.config.bucket)
            .key(path)
            .send()
            .await
            .map_err(|e| ApiError::biz(format!("S3 删除失败: {}", e)))?;
        Ok(())
    }

    async fn get_content(&self, path: &str) -> ApiResult<Vec<u8>> {
        let output = self
            .client
            .get_object()
            .bucket(&self.config.bucket)
            .key(path)
            .send()
            .await
            .map_err(|e| ApiError::biz(format!("S3 读取失败: {}", e)))?;

        let bytes = output
            .body
            .collect()
            .await
            .map_err(|e| ApiError::biz(format!("S3 读取流失败: {}", e)))?
            .into_bytes();
        Ok(bytes.to_vec())
    }

    async fn presign_put_url(&self, path: &str) -> ApiResult<String> {
        let presigning_config = aws_sdk_s3::presigning::PresigningConfig::expires_in(
            std::time::Duration::from_secs(60 * 10),
        ) // 10 mins
        .map_err(|e| ApiError::biz(format!("S3 预签名配置失败: {}", e)))?;

        let presigned_request = self
            .client
            .put_object()
            .bucket(&self.config.bucket)
            .key(path)
            .presigned(presigning_config)
            .await
            .map_err(|e| ApiError::biz(format!("S3 预签名失败: {}", e)))?;

        Ok(presigned_request.uri().to_string())
    }
}

// ================== FTP File Client ==================

pub struct FtpFileClient {
    config: FtpFileClientConfig,
}

impl FtpFileClient {
    pub fn new(config: FtpFileClientConfig) -> Self {
        Self { config }
    }

    // Helper to run sync FTP operations
    async fn run_sync<F, R>(&self, f: F) -> ApiResult<R>
    where
        F: FnOnce(&mut suppaftp::FtpStream) -> ApiResult<R> + Send + 'static,
        R: Send + 'static,
    {
        let config = self.config.clone(); // Clone for the thread
        tokio::task::spawn_blocking(move || {
            let mut ftp = suppaftp::FtpStream::connect(format!("{}:{}", config.host, config.port))
                .map_err(|e| ApiError::biz(format!("FTP 连接失败: {}", e)))?;

            ftp.login(&config.username, &config.password)
                .map_err(|e| ApiError::biz(format!("FTP 登录失败: {}", e)))?;

            f(&mut ftp)
        })
        .await
        .map_err(|e| ApiError::biz(format!("FTP 任务执行错误: {}", e)))?
    }
}

#[async_trait::async_trait]
impl FileClient for FtpFileClient {
    async fn upload(&self, content: &[u8], path: &str, _content_type: &str) -> ApiResult<String> {
        let content = content.to_vec();
        let path_str = path.to_string();
        let base_path = self.config.base_path.clone();

        let path_clone = path_str.clone();
        self.run_sync(move |ftp| {
            let full_path = format!("{}/{}", base_path.trim_end_matches('/'), path_clone);
            if let Some(parent) = Path::new(&full_path).parent() {
                let parent_str = parent.to_string_lossy();
                let _ = ftp.mkdir(&parent_str);
            }

            ftp.put_file(&full_path, &mut &content[..])
                .map_err(|e| ApiError::biz(format!("FTP 上传失败: {}", e)))?;
            Ok(())
        })
        .await?;

        Ok(format!("{}/{}", self.config.domain, path_str))
    }

    async fn delete(&self, path: &str) -> ApiResult<()> {
        let path = path.to_string();
        let base_path = self.config.base_path.clone();
        self.run_sync(move |ftp| {
            let full_path = format!("{}/{}", base_path.trim_end_matches('/'), path);
            ftp.rm(&full_path)
                .map_err(|e| ApiError::biz(format!("FTP 删除失败: {}", e)))?;
            Ok(())
        })
        .await
    }

    async fn get_content(&self, path: &str) -> ApiResult<Vec<u8>> {
        let path = path.to_string();
        let base_path = self.config.base_path.clone();
        self.run_sync(move |ftp| {
            let full_path = format!("{}/{}", base_path.trim_end_matches('/'), path);
            let bytes = ftp
                .retr_as_buffer(&full_path)
                .map_err(|e| ApiError::biz(format!("FTP 读取失败: {}", e)))?
                .into_inner();
            Ok(bytes)
        })
        .await
    }
}

// ================== SFTP File Client ==================

pub struct SftpFileClient {
    config: SftpFileClientConfig,
}

impl SftpFileClient {
    pub fn new(config: SftpFileClientConfig) -> Self {
        Self { config }
    }

    async fn run_sync<F, R>(&self, f: F) -> ApiResult<R>
    where
        F: FnOnce(&ssh2::Session, &ssh2::Sftp) -> ApiResult<R> + Send + 'static,
        R: Send + 'static,
    {
        let config = self.config.clone();
        tokio::task::spawn_blocking(move || {
            let tcp = TcpStream::connect(format!("{}:{}", config.host, config.port))
                .map_err(|e| ApiError::biz(format!("SFTP TCP 连接失败: {}", e)))?;

            let mut sess = ssh2::Session::new()
                .map_err(|e| ApiError::biz(format!("SFTP Session 创建失败: {}", e)))?;
            sess.set_tcp_stream(tcp);
            sess.handshake()
                .map_err(|e| ApiError::biz(format!("SFTP Handshake 失败: {}", e)))?;

            sess.userauth_password(&config.username, &config.password)
                .map_err(|e| ApiError::biz(format!("SFTP 认证失败: {}", e)))?;

            let sftp = sess
                .sftp()
                .map_err(|e| ApiError::biz(format!("SFTP 初始化失败: {}", e)))?;

            f(&sess, &sftp)
        })
        .await
        .map_err(|e| ApiError::biz(format!("SFTP 任务执行错误: {}", e)))?
    }
}

#[async_trait::async_trait]
impl FileClient for SftpFileClient {
    async fn upload(&self, content: &[u8], path: &str, _content_type: &str) -> ApiResult<String> {
        let content = content.to_vec();
        let path_str = path.to_string();
        let base_path = self.config.base_path.clone();

        let path_clone = path_str.clone();
        self.run_sync(move |_sess, sftp| {
            let full_path = Path::new(&base_path).join(&path_clone);

            if let Some(parent) = full_path.parent() {
                let _ = sftp.mkdir(parent, 0o755);
            }

            let mut file = sftp
                .create(&full_path)
                .map_err(|e| ApiError::biz(format!("SFTP 创建文件失败: {}", e)))?;

            file.write_all(&content)
                .map_err(|e| ApiError::biz(format!("SFTP 写入文件失败: {}", e)))?;

            Ok(())
        })
        .await?;

        Ok(format!("{}/{}", self.config.domain, path_str))
    }

    async fn delete(&self, path: &str) -> ApiResult<()> {
        let path = path.to_string();
        let base_path = self.config.base_path.clone();
        self.run_sync(move |_sess, sftp| {
            let full_path = Path::new(&base_path).join(&path);
            sftp.unlink(&full_path)
                .map_err(|e| ApiError::biz(format!("SFTP 删除失败: {}", e)))?;
            Ok(())
        })
        .await
    }

    async fn get_content(&self, path: &str) -> ApiResult<Vec<u8>> {
        let path = path.to_string();
        let base_path = self.config.base_path.clone();
        self.run_sync(move |_sess, sftp| {
            let full_path = Path::new(&base_path).join(&path);
            let mut file = sftp
                .open(&full_path)
                .map_err(|e| ApiError::biz(format!("SFTP 打开文件失败: {}", e)))?;
            let mut content = Vec::new();
            use std::io::Read;
            file.read_to_end(&mut content)
                .map_err(|e| ApiError::biz(format!("SFTP 读取文件失败: {}", e)))?;
            Ok(content)
        })
        .await
    }
}

// ================== Factory ==================

pub async fn create_file_client(
    config_id: String,
    storage: &FileStorageEnum,
    config: &Json,
) -> ApiResult<Box<dyn FileClient>> {
    match storage {
        FileStorageEnum::DB => {
            let config = validate_and_parse::<DbFileClientConfig>(config)?;
            Ok(Box::new(DbFileClient::new(config_id, config)))
        }
        FileStorageEnum::LOCAL => {
            let config = validate_and_parse::<LocalFileClientConfig>(config)?;
            Ok(Box::new(LocalFileClient::new(config)))
        }
        FileStorageEnum::FTP => {
            let config = validate_and_parse::<FtpFileClientConfig>(config)?;
            Ok(Box::new(FtpFileClient::new(config)))
        }
        FileStorageEnum::SFTP => {
            let config = validate_and_parse::<SftpFileClientConfig>(config)?;
            Ok(Box::new(SftpFileClient::new(config)))
        }
        FileStorageEnum::S3 => {
            let config = validate_and_parse::<S3FileClientConfig>(config)?;
            Ok(Box::new(S3FileClient::new(config)))
        }
    }
}
