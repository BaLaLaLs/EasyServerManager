use std::fs::File;
use std::io::Read;
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: Server,
    pub database: Database
}
#[derive(Debug, Deserialize)]
pub struct Server {
    pub address: String,
    pub port: u16,
    pub jwt_secret: String,
}
#[derive(Debug, Deserialize)]
pub struct Database {
    pub db_driver: String,
    pub url: String,
}

//  只要是配置文件中的配置项，都可以通过这个结构体来获取，
// 只要读取一次值后保存到内存，一直可供使用
pub static GLOBAL_CONFIGS: Lazy<AppConfig> = Lazy::new(AppConfig::init);
impl AppConfig {
    pub fn init() -> Self {
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
}