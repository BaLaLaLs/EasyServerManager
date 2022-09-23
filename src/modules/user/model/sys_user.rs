use rbatis::crud;
use rbatis::rbdc::datetime::FastDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysUser {
    pub id: u32,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub created_at: Option<FastDateTime>,
}
crud!(SysUser{});