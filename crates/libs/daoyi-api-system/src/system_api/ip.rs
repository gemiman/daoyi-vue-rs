use axum::{Router, debug_handler, routing};
use daoyi_common_support::app::AppState;
use daoyi_common_support::request::valid::ValidQuery;
use daoyi_common_support::response::{ApiResponse, RestApiResult};
use daoyi_common_support::utils::area_utils::{AreaUtils, ID_CHINA};
use daoyi_common_support::utils::ip_utils::IPUtils;
use daoyi_common_support::vo::ip_vo::{AreaNodeRespVO, IpParams};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/tree", routing::get(get_area_tree))
        .route("/get-by-ip", routing::get(get_area_by_ip))
}

#[debug_handler]
async fn get_area_tree() -> RestApiResult<Vec<AreaNodeRespVO>> {
    let tree = build_area_tree(ID_CHINA);
    ApiResponse::success(tree)
}

#[debug_handler]
async fn get_area_by_ip(
    ValidQuery(IpParams { ip }): ValidQuery<IpParams>,
) -> RestApiResult<String> {
    let area = IPUtils::get_simple_region(&ip);
    ApiResponse::success(area)
}

fn build_area_tree(parent_id: &str) -> Vec<AreaNodeRespVO> {
    let mut nodes = Vec::new();
    if let Some(children_ids) = AreaUtils::get_children(parent_id) {
        for id in children_ids {
            if let Some(area) = AreaUtils::get_area(id) {
                nodes.push(AreaNodeRespVO {
                    id: area.id.clone(),
                    name: area.name.clone(),
                    children: build_area_tree(&area.id),
                });
            }
        }
    }
    nodes
}
