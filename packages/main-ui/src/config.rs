use tracing::Level;

#[derive(Debug)]
pub struct KakaoConfig {
    pub client_id: &'static str,
}

#[derive(Debug)]
pub struct Config {
    pub env: &'static str,
    pub domain: &'static str,
    pub log_level: Level,
    pub main_api_endpoint: String,
    pub kakao: KakaoConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            env: option_env!("ENV").expect("You must set ENV"),
            domain: option_env!("DOMAIN").expect("You must set DOMAIN"),
            log_level: match option_env!("RUST_LOG") {
                Some("trace") => Level::TRACE,
                Some("debug") => Level::DEBUG,
                Some("info") => Level::INFO,
                Some("warn") => Level::WARN,
                Some("error") => Level::ERROR,
                _ => Level::INFO,
            },
            main_api_endpoint: option_env!("MAIN_API_ENDPOINT")
                .unwrap_or("https://localhost:3000")
                .to_string(),
            kakao: KakaoConfig {
                client_id: option_env!("KAKAO_CLIENT_ID").expect("You must set KAKAO_CLIENT_ID"),
            },
        }
    }
}

static mut CONFIG: Option<Config> = None;

#[allow(static_mut_refs)]
pub fn get() -> &'static Config {
    unsafe {
        if CONFIG.is_none() {
            CONFIG = Some(Config::default());
        }
        CONFIG.as_ref().unwrap()
    }
}
