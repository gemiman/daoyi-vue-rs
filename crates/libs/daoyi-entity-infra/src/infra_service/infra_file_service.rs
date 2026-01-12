use crate::infra_entity::{infra_file, prelude::*};
use crate::infra_service::infra_file_config_service::{get_file_client, get_master_file_client};
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::models::pagination::PageResult;
use daoyi_common_support::vo::infra_vo::{
    FileCreateReqVO, FilePageReqVO, FilePresignedUrlRespVO, FileRespVO,
};
use daoyi_common_support::{database, id_util};
use daoyi_macros::transactional;
use sea_orm::sqlx::types::chrono::Local;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, PaginatorTrait, QueryFilter, QueryOrder, QueryTrait, Set,
};

async fn gen_filename(path: Option<&str>, name: &str) -> ApiResult<String> {
    let filename = if let Some(p) = path {
        String::from(p)
    } else {
        // Generate path: yyyy/MM/dd/uuid.ext
        let now = Local::now();
        let uuid = id_util::xid();
        let ext = std::path::Path::new(name)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        format!(
            "{}/{}/{}/{}.{}",
            now.format("%Y"),
            now.format("%m"),
            now.format("%d"),
            uuid,
            ext
        )
    };
    Ok(filename)
}
pub async fn create_file(
    name: String,
    path: Option<String>,
    content: Vec<u8>,
    content_type: String,
) -> ApiResult<String> {
    let (client, config_id) = get_master_file_client().await?;

    let filename = gen_filename(path.as_deref(), &name).await?;

    let content_len = content.len() as i32;
    // Upload
    let url = client.upload(content, &filename, &content_type).await?;

    // Save to DB
    let db = database::get_db_async().await;
    let model = infra_file::ActiveModel {
        id: Set(id_util::xid()),
        config_id: Set(Some(config_id)),
        name: Set(Some(name)),
        path: Set(filename),
        url: Set(url.clone()),
        r#type: Set(Some(content_type)),
        size: Set(content_len),
        ..Default::default()
    };
    model.insert(&db).await?;

    Ok(url)
}

pub async fn create_file_from_req(req: FileCreateReqVO) -> ApiResult<String> {
    let db = database::get_db_async().await;
    let config_id = if let Some(cid) = req.config_id {
        cid
    } else {
        let (_, cid) = get_master_file_client().await?;
        cid
    };

    let model = infra_file::ActiveModel {
        config_id: Set(Some(config_id)),
        name: Set(Some(req.name)),
        path: Set(req.path),
        url: Set(req.url),
        r#type: Set(req.r#type),
        size: Set(req.size),
        ..Default::default()
    };
    let res = model.insert(&db).await?;
    Ok(res.id)
}

pub async fn delete_file(id: &str) -> ApiResult<()> {
    let db = database::get_db_async().await;
    let file = InfraFile::find_by_id_perm_with_tenant(&db, id)
        .await?
        .ok_or_else(|| ApiError::biz("文件不存在"))?;

    // Delete from Client
    if let Some(config_id) = &file.config_id {
        // We catch error here? Java: propagates exception.
        let client = get_file_client(config_id).await?;
        client.delete(&file.path).await?;
    }

    // Delete from DB
    InfraFile::delete_logical_by_id(&db, id).await?;
    Ok(())
}

#[transactional]
pub async fn delete_file_list(ids: &Vec<String>) -> ApiResult<()> {
    // Ideally parallelize, but loop is fine for now
    for id in ids {
        delete_file(id).await?;
    }
    Ok(())
}

pub async fn get_file_content(config_id: &str, path: &str) -> ApiResult<Vec<u8>> {
    let client = get_file_client(config_id).await?;
    client.get_content(path).await
}

pub async fn presign_put_url(
    name: String,
    path: Option<String>,
) -> ApiResult<FilePresignedUrlRespVO> {
    let (client, _config_id) = get_master_file_client().await?;

    let filename = gen_filename(path.as_deref(), &name).await?;

    let upload_url = client.presign_put_url(&filename).await?;

    Ok(FilePresignedUrlRespVO {
        upload_url,
        url: filename, // Ideally this should be full URL, but we lack domain access in trait
    })
}

pub async fn get_file_page(params: &FilePageReqVO) -> ApiResult<PageResult<FileRespVO>> {
    let db = database::get_db_async().await;
    let paginator = InfraFile::find_perm_with_tenant()
        .await
        .apply_if(params.path.as_ref(), |query, val| {
            query.filter(infra_file::Column::Path.contains(val))
        })
        .apply_if(params.r#type.as_ref(), |query, val| {
            query.filter(infra_file::Column::Type.eq(val))
        })
        .apply_if(params.create_time.as_ref(), |query, val| {
            query.filter(infra_file::Column::CreateTime.between(val[0], val[1]))
        })
        .order_by_desc(infra_file::Column::CreateTime)
        .paginate(&db, params.pagination.page_size);

    let total = paginator.num_items().await?;
    let list = paginator
        .fetch_page(params.pagination.page_no - 1)
        .await?
        .into_iter()
        .map(|m| m.into())
        .collect();
    let page = PageResult::from_pagination(&params.pagination, total, list);
    Ok(page)
}
