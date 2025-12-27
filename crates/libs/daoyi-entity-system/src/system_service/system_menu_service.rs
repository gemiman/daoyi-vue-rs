use crate::system_entity::prelude::*;
use crate::system_entity::system_menu;
use daoyi_common_support::database;
use daoyi_common_support::enumeration::MenuTypeEnum;
use daoyi_common_support::error::ApiResult;
use daoyi_common_support::vo::system_vo::MenuVO;
use sea_orm::QueryTrait;
use sea_orm::prelude::*;
use std::collections::HashMap;

pub async fn get_menu_list(ids: Option<&Vec<String>>) -> ApiResult<Vec<system_menu::Model>> {
    if ids.is_some() && ids.unwrap().is_empty() {
        return Ok(vec![]);
    }
    let db = database::get().await;
    Ok(SystemMenu::find_perm()
        .await
        .apply_if(ids, |query, ids| {
            query.filter(system_menu::Column::Id.is_in(ids))
        })
        .all(db)
        .await?)
}

pub async fn build_menu_tree(menus: Vec<system_menu::Model>) -> ApiResult<Vec<MenuVO>> {
    let mut menus = menus;
    menus.retain(|m| m.r#type != MenuTypeEnum::BUTTON);

    menus.sort_by(|a, b| a.sort.cmp(&b.sort));

    let mut map: HashMap<String, Vec<system_menu::Model>> = HashMap::new();
    for menu in menus {
        map.entry(menu.parent_id.clone())
            .or_insert(vec![])
            .push(menu);
    }

    Ok(build_children(String::from("0"), &map))
}

fn build_children(
    parent_id: String,
    map: &HashMap<String, Vec<system_menu::Model>>,
) -> Vec<MenuVO> {
    if let Some(children) = map.get(&parent_id) {
        children
            .iter()
            .map(|m| {
                let m = m.to_owned();
                MenuVO {
                    id: m.id.clone(),
                    parent_id: m.parent_id,
                    name: m.name,
                    path: m.path,
                    component: m.component,
                    component_name: m.component_name,
                    icon: m.icon,
                    visible: m.visible,
                    keep_alive: m.keep_alive,
                    always_show: m.always_show,
                    children: build_children(m.id, map),
                }
            })
            .collect()
    } else {
        vec![]
    }
}
