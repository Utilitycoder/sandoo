pub static PROJECT_NAME: &str = "sandoo";

// Function that loads the environment variables as a string
pub fn get_env(key: &str) -> String {
    std::env::var(key).unwrap_or(String::from(""))
}

#[derive(Debug, Clone)]
pub struct Env {
    pub http_url: String,
    pub wss_url: String,
    pub bot_address: String,
    pub private_key: String,
    pub identity_key: String,
    pub telegram_token: String,
    pub telegram_chat_id: String,
    pub use_alert: bool,
    pub debug: bool,
}

// Creating new instance of Env will automatically load the environment variables
impl Env {
    pub fn new() -> Self {
        let http_url = get_env("HTTP_URL");
        let wss_url = get_env("WSS_URL");
        let bot_address = get_env("BOT_ADDRESS");
        let private_key = get_env("PRIVATE_KEY");
        let identity_key = get_env("IDENTITY_KEY");
        let telegram_token = get_env("TELEGRAM_TOKEN");
        let telegram_chat_id = get_env("TELEGRAM_CHAT_ID");
        let use_alert = get_env("USE_ALERT").parse::<bool>().unwrap_or(false);
        let debug = get_env("DEBUG").parse::<bool>().unwrap_or(false);

        Env {
            http_url,
            wss_url,
            bot_address,
            private_key,
            identity_key,
            telegram_token,
            telegram_chat_id,
            use_alert,
            debug,
        }
    }
}

// Flashbot builder
pub static COINBASE: &str = "0xDAFEA492D9c6733ae3d56b7Ed1ADB60692c98Bc5";

pub static WETH: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
pub static WETH_BALANCE_SLOT: i32 = 3;
pub static WETH_DECIMALS: u8 = 18;
