#[tokio::main]
async fn main() -> anyhow::Result<()> {
    daoyi_common_support::app::run(Some(env!("CARGO_PKG_NAME")), daoyi_server::create_router())
        .await
}
