use daoyi_common_support::enumeration::FileStorageEnum;
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::serde::validate_and_parse;
use daoyi_common_support::vo::infra_vo::LocalFileClientConfig;
use sea_orm::prelude::async_trait;
use sea_orm::prelude::Json;

#[async_trait::async_trait]
pub trait FileClient: Send + Sync {
    async fn upload(&self, content: &[u8], path: &str, content_type: &str) -> ApiResult<String>;
}

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
    async fn upload(&self, _content: &[u8], path: &str, _content_type: &str) -> ApiResult<String> {
        // TODO: Implement actual file writing to local disk
        Ok(format!("{}/{}", self.config.domain, path))
    }
}

pub fn create_file_client(
    storage: &FileStorageEnum,
    config: &Json,
) -> ApiResult<Box<dyn FileClient>> {
    match storage {
        FileStorageEnum::DB => Err(ApiError::biz("暂不支持 DB 存储")),
        FileStorageEnum::LOCAL => {
            let config = validate_and_parse::<LocalFileClientConfig>(config)?;
            Ok(Box::new(LocalFileClient::new(config)))
        }
        FileStorageEnum::FTP => Err(ApiError::biz("暂不支持 FTP 存储")),
        FileStorageEnum::SFTP => Err(ApiError::biz("暂不支持 SFTP 存储")),
        FileStorageEnum::S3 => Err(ApiError::biz("暂不支持 S3 存储")),
    }
}
