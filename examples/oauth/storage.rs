pub fn save_access_token(json: String) {
    std::fs::write(".oauth.cache.json", json).expect("Unable to save access token")
}

pub fn read_access_token() -> std::io::Result<String> {
    std::fs::read_to_string(".oauth.cache.json")
}
