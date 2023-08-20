pub fn save_access_token(json: String) {
    std::fs::write(".token.cache.json", json).expect("Unable to save access token")
}

pub fn load_access_token() -> std::io::Result<String> {
    std::fs::read_to_string(".token.cache.json")
}
