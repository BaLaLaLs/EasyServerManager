mod modules;


use std::fs::File;
use std::io::Read;
use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use std::str::FromStr;

#[tokio::main]
async fn main() {
    let configs = load_config();
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let address = format!("{}:{}",configs.server.address.as_str(), configs.server.port.to_string().as_str());
    let addr = SocketAddr::from_str(address.as_str()).unwrap();
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
fn load_config() -> modules::config::configs::Configs {
    let mut config_path = String::from("Config.toml");
    if let Some(s) = std::env::var_os("ESM_CONFIG_PATH").as_mut() {
        config_path = s.to_str().unwrap_or_default().to_string();
    }
    let mut file= match File::open(config_path.as_str()) {
        Ok(f) => f,
        Err(e) => panic!("配置文件不存在 错误信息：{}", e),
    };
    let mut cfg_contents = String::new();
    match file.read_to_string(&mut cfg_contents) {
        Ok(s) => s,
        Err(e) => panic!("读取配置文件失败，错误信息：{}", e),
    };
    toml::from_str(&cfg_contents).expect("无法解析配置文件!")
}