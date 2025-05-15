use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_server_addr")]
    pub port: String,

    pub solana_rpc_url: String,
    pub solana_keypair: String,

    #[serde(default = "default_log_level")]
    pub log_level: String,
}

fn default_server_addr() -> String {
    String::from("127.0.0.1:8080")
}

fn default_log_level() -> String {
    String::from("info")
}

pub static SETTINGS: Lazy<Config> = Lazy::new(|| {
    dotenv::dotenv().ok();

    let cfg = envy::from_env::<Config>()
        .expect("Failed to load configuration from environment variables");

    std::env::set_var("RUST_LOG", &cfg.log_level);
    env_logger::init();

    cfg
});
