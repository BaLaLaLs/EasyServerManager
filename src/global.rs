use std::borrow::Borrow;
use std::future::Future;
use std::sync::{Arc, RwLock};
use tokio::sync::Mutex;
use std::thread;
use std::time::Duration;
use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;
use rbatis::Rbatis;
use rbdc_mssql::driver::MssqlDriver;
use rbdc_mysql::driver::MysqlDriver;
use rbdc_pg::driver::PgDriver;
use rbdc_sqlite::driver::SqliteDriver;
use crate::{global, GLOBAL_CONFIGS};
use serde::{Deserialize, Serialize};
use sysinfo::{System, SystemExt};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub(crate) sub: String,
    pub(crate) company: String,
    pub(crate) exp: usize,
    pub(crate) username: String
}
pub struct Keys {
    pub(crate) encoding: EncodingKey,
    pub(crate) decoding: DecodingKey,
}
impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub static JWT_KEYS: Lazy<Keys> = Lazy::new(|| { Keys::new(GLOBAL_CONFIGS.server.jwt_secret.as_bytes()) });
pub static DB: Lazy<Rbatis> = Lazy::new(init_db);
pub static SYSTEM: Lazy<Arc<Mutex<System>>> = Lazy::new(|| {
    let arc = Arc::new(Mutex::new(System::new_all()));
    let mut arc_clone = arc.clone();
    tokio::task::spawn(async move {
        loop {
            thread::sleep(Duration::new(1, 0));
            arc_clone.lock().await
            .refresh_all();
            // arc_clone.lock().await
            //     .refresh_disks_list();
        }
    });
    arc
});

#[macro_export]
macro_rules! pool {
    () => {
        global::DB.clone().borrow_mut()
    };
}
fn init_db() -> Rbatis {
    let rb = Rbatis::new();
    match &GLOBAL_CONFIGS.database.db_driver.clone() as &str {
        "sqlite" => rb.init(SqliteDriver {}, GLOBAL_CONFIGS.database.url.as_str()).unwrap(),
        "mysql" => rb.init(MysqlDriver {}, GLOBAL_CONFIGS.database.url.as_str()).unwrap(),
        "mssql" => rb.init(MssqlDriver {}, GLOBAL_CONFIGS.database.url.as_str()).unwrap(),
        "pg" => rb.init(PgDriver {}, GLOBAL_CONFIGS.database.url.as_str()).unwrap(),
        s => panic!("not supported {} driver!", s)
    };
    println!("pool status: {:?}", rb.get_pool().expect("pool not init!").status());
    rb
}