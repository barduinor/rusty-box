//! Configuration for the Box API client.
/// Configuration structure for the Box API.
#[derive(Debug, Clone, Serialize)]
pub struct Config {
    base_api_url: String,
    upload_url: String,
    pub oauth2_api_url: String,
    pub oauth2_authorize_url: String,
    pub api_version: String,
    pub max_retry_attempts: u8,
    pub chunk_upload_threads: u8,
    pub user_agent: String,
}

//     BASE_API_URL = 'https://api.box.com/2.0'
//     UPLOAD_URL = 'https://upload.box.com/api/2.0'
//     OAUTH2_API_URL = 'https://api.box.com/oauth2'
//     OAUTH2_AUTHORIZE_URL = 'https://account.box.com/api/oauth2/authorize'
//     MAX_RETRY_ATTEMPTS = 5
//     CHUNK_UPLOAD_THREADS = 5

impl Default for Config {
    fn default() -> Self {
        Config {
            base_api_url: String::from("https://api.box.com"),
            upload_url: String::from("https://upload.box.com/api"),
            oauth2_api_url: String::from("https://api.box.com/oauth2"),
            oauth2_authorize_url: String::from("https://account.box.com/api/oauth2/authorize"),
            api_version: String::from("2.0"),
            max_retry_attempts: 5,
            chunk_upload_threads: 5,
            user_agent: "box-rust-sdk/rusty-box".to_owned(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }
    pub fn base_api_url(&self) -> String {
        format!("{}/{}", self.base_api_url, self.api_version)
    }
    // pub fn set_base_api_url(&mut self, base_api_url: String) {
    //     self.base_api_url = base_api_url;
    // }
    pub fn upload_url(&self) -> String {
        format!("{}/{}", self.upload_url, self.api_version)
    }
    pub fn set_upload_url(&mut self, upload_url: String) {
        self.upload_url = upload_url;
    }

    pub fn user_agent(&self) -> String {
        self.user_agent.clone()
    }
    // pub fn oauth2_api_url(&self) -> String {
    //     self.oauth2_api_url.clone()
    // }
    // pub fn oauth2_authorize_url(&self) -> String {
    //     self.oauth2_authorize_url.clone()
    // }
    // pub fn max_retry_attempts(&self) -> u8 {
    //     self.max_retry_attempts
    // }
    // pub fn set_max_retry_attempts(&mut self, max_retry_attempts: u8) {
    //     self.max_retry_attempts = max_retry_attempts;
    // }
    // pub fn chunk_upload_threads(&self) -> u8 {
    //     self.chunk_upload_threads
    // }
    // pub fn set_chunk_upload_threads(&mut self, chunk_upload_threads: u8) {
    //     self.chunk_upload_threads = chunk_upload_threads;
    // }
    // pub fn api_version(&self) -> String {
    //     self.api_version.clone()
    // }
    // pub fn set_api_version(&mut self, api_version: String) {
    //     self.api_version = api_version;
    // }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn test_default_config_values() {
        let config = Config::default();

        assert_eq!(config.base_api_url, "https://api.box.com");
        assert_eq!(config.base_api_url(), "https://api.box.com/2.0");

        assert_eq!(config.upload_url, "https://upload.box.com/api");
        assert_eq!(config.upload_url(), "https://upload.box.com/api/2.0");

        assert_eq!(config.oauth2_api_url, "https://api.box.com/oauth2");
        assert_eq!(
            config.oauth2_authorize_url,
            "https://account.box.com/api/oauth2/authorize"
        );
        assert_eq!(config.max_retry_attempts, 5);
        assert_eq!(config.chunk_upload_threads, 5);
        assert_eq!(config.api_version, "2.0");
        assert_eq!(config.user_agent, "box-rust-sdk/rusty-box".to_owned());
    }

    #[test]
    fn test_config_values_v3() {
        let config = Config {
            api_version: String::from("3.0"),
            ..Default::default()
        };
        assert_eq!(config.base_api_url(), "https://api.box.com/3.0");
        assert_eq!(config.upload_url(), "https://upload.box.com/api/3.0");
    }
}
