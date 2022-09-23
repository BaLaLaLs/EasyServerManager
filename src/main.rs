extern crate core;

use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;
use axum::{Extension, http::StatusCode, Json, response::IntoResponse, Router, routing::{get, post}};
use colored::Colorize;
use rbatis::Rbatis;
use rbatis::rbdc::db::Driver;
use rbdc_mssql::driver::MssqlDriver;
use rbdc_mysql::driver::MysqlDriver;
use rbdc_pg::driver::PgDriver;
use rbdc_sqlite::driver::SqliteDriver;
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tracing::log;
use tracing_subscriber::fmt::format;

use crate::modules::config::configs::GLOBAL_CONFIGS;
use crate::modules::user::model::sys_user::SysUser;
use crate::my_env::print_banner;

mod modules;
mod my_env;

#[tokio::main]
async fn main() {
    print_banner();
    let mut db: Rbatis = init_db().await;
    let x = &mut db;
    let vec = SysUser::select_all(&mut db).await.unwrap();

    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(root))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(Box::new(db))),
        );
    let address = format!("{}:{}",GLOBAL_CONFIGS.server.address.as_str(), GLOBAL_CONFIGS.server.port.to_string().as_str());
    let addr = SocketAddr::from_str(address.as_str()).unwrap();
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root(Extension(mut db): Extension<Box<Rbatis>>) -> impl IntoResponse {
    Json(SysUser::select_all(db.as_mut()).await.unwrap())
}
async fn init_db() -> Rbatis {
    let rb = Rbatis::new();
    match &GLOBAL_CONFIGS.database.db_driver.clone() as &str {
        "sqlite" => rb.init(SqliteDriver{}, GLOBAL_CONFIGS.database.url.as_str()).unwrap(),
        "mysql" => rb.init(MysqlDriver{}, GLOBAL_CONFIGS.database.url.as_str()).unwrap(),
        "mssql" => rb.init(MssqlDriver {}, GLOBAL_CONFIGS.database.url.as_str()).unwrap(),
        "pg" => rb.init(PgDriver {}, GLOBAL_CONFIGS.database.url.as_str()).unwrap(),
        s => panic!("not supported {} driver!", s)
    };
    rb
}