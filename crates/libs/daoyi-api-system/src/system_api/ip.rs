use axum::{Router, debug_handler, routing};
use daoyi_common_support::app::AppState;
use daoyi_common_support::request::valid::ValidQuery;
use daoyi_common_support::response::RestApiResult;
use daoyi_common_support::vo::ip_vo::{AreaNodeRespVO, IpParams};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/tree", routing::get(get_area_tree))
        .route("/get-by-ip", routing::get(get_area_by_ip))
}

#[debug_handler]
async fn get_area_tree() -> RestApiResult<Vec<AreaNodeRespVO>> {
    todo!()
}

#[debug_handler]
async fn get_area_by_ip(
    ValidQuery(IpParams { ip }): ValidQuery<IpParams>,
) -> RestApiResult<String> {
    todo!()
}
