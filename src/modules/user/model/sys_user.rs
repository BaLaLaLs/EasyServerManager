use rbatis::{crud, impl_select};
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
impl_select!(SysUser{select_by_username_password(username:&str,password:&str) -> Option => "`where username = #{username} and password = #{password} limit 1`"});