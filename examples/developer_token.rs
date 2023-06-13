// use cargo run --example users_main to run this file
// use dotenv;

use box_rust_sdk::rest_api::{
    api::models::{api_config::ApiConfig, api_configuration_old},
    users::users_api,
};
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    let developer_token = env::var("DEVELOPER_TOKEN").expect("DEVELOPER_TOKEN not set");

    let api_config = ApiConfig::default();

    let mut client_config = api_configuration_old::Configuration::new();
    client_config.base_path = api_config.base_api_url();
    client_config.oauth_access_token = Some(developer_token.clone());

    let params = users_api::GetUsersMeParams::default();

    let user = users_api::get_users_me(&client_config, params)
        .await
        .expect("401");

    println!("Current user:\n{user:#?}\n");
}
