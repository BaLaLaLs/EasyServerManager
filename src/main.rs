extern crate core;

use std::borrow::{Borrow, BorrowMut};
use std::net::SocketAddr;
use std::ops::{Deref, DerefMut};

use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use axum::{Extension, TypedHeader};
use axum::http::{HeaderValue, Method, StatusCode};
use axum::Json;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use axum::routing::post;
use axum::extract::{Query, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use chrono::Local;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};


use rbatis::Rbatis;

use rbdc_mssql::driver::MssqlDriver;
use rbdc_mssql::tiberius::time::chrono::Utc;
use rbdc_mysql::driver::MysqlDriver;
use rbdc_pg::driver::PgDriver;
use rbdc_sqlite::driver::SqliteDriver;
use serde::{Deserialize};
use sysinfo::{System, SystemExt};
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::cors::CorsLayer;
use crate::global::{Claims, SYSTEM};
use crate::modules::config;

use crate::modules::config::configs::GLOBAL_CONFIGS;
use crate::modules::user::model::sys_user::SysUser;
use crate::modules::ws::ws_protocol::{MsgType, WsMessage};
use crate::my_env::print_banner;

mod modules;
mod my_env;
mod global;

#[tokio::main]
async fn main() {

    print_banner();
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .nest("/user",modules::user::sys_user_route::api())
        .route("/ws", get(ws_handler))
        .layer(
               CorsLayer::new()
                   .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                   .allow_methods([Method::GET])
        );
    let address = format!("{}:{}", GLOBAL_CONFIGS.server.address.as_str(), GLOBAL_CONFIGS.server.port.to_string().as_str());
    let addr = SocketAddr::from_str(address.as_str()).unwrap();
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

pub struct AppState {
    db: Rbatis,
    system: Arc<Mutex<System>>
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        println!("`{}` connected", user_agent.as_str());
    }

    ws.on_upgrade(handle_socket)
}
async fn handle_socket(mut socket: WebSocket) {

    loop {
        let arc = Arc::clone(SYSTEM.deref());
        let result = arc.lock().await;
        let ext = modules::system::system_ext::SysInfoExt::new(&result);
        let msg = WsMessage { msg_type: MsgType::SystemStatus, data: &ext };
        if socket
            .send(Message::Text(serde_json::to_string(&msg).unwrap()))
            .await
            .is_err()
        {
            println!("client disconnected");
            return;
        }
        tokio::time::sleep(Duration::from_secs(3)).await;
    }
}