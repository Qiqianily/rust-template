use template::{
    conf::app::AppConfig,
    db::{get_global_database_pool, pgsql::init_database_pool_with_config, set_global_db},
    log::logger::{init_logger_with_file, init_logger_without_file},
    pb::{
        explanation::explanation_hu_service_server::ExplanationHuServiceServer,
        greeter::greeter_service_server::GreeterServiceServer,
    },
    service_impl::{explanation::ExplanationHuServiceImpl, greeter::GreeterServiceImpl},
};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let addr = "[::1]:50051".parse()?;
    let greeter_service = GreeterServiceImpl::default();

    // println!("Starting UserService on {}", addr);

    // Server::builder()
    //     .add_service(GreeterServiceServer::new(greeter_service))
    //     .serve(addr)
    //     .await?;
    //
    // 1. 读取配置信息
    let config = AppConfig::load()?;
    let log_level = config.base().log_level();
    // 2. 初始化日志
    if config.is_log_file() {
        let _guard = init_logger_with_file(log_level).await?;
    } else {
        init_logger_without_file(log_level).await?;
    }
    // 3. 初始化数据库连接池
    let db = init_database_pool_with_config(config.database()).await?;
    set_global_db(db).await?;
    // 4. 创建服务
    let srv = ExplanationHuServiceImpl::new(get_global_database_pool());
    // 服务地址
    let addr = format!("[::1]:{}", config.base().port()).parse()?;
    tracing::info!("Starting UserService on {}", addr);
    Server::builder()
        .add_service(GreeterServiceServer::new(greeter_service))
        .add_service(ExplanationHuServiceServer::new(srv))
        .serve(addr)
        .await?;
    Ok(())
}
