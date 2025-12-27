use crate::system_entity::prelude::*;
use daoyi_common_support::error::ApiResult;
use sea_orm::prelude::*;
pub async fn get_role_menu_list_by_role_id(role_ids: &Vec<String>) -> ApiResult<Vec<String>> {
    if role_ids.is_empty() {
        return Ok(vec![]);
    }
    // 如果是管理员的情况下，获取全部菜单编号
    todo!()
}
