use rbatis::Rbatis;
use crate::SysUser;

pub async fn list_sys_user(db: &mut Rbatis) -> Vec<SysUser> {
    SysUser::select_all(db).await.unwrap()
}