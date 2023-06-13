// use cargo run --example users_main to run this file
// use dotenv;

use box_rust_sdk::rest_api::{
    api::{
        api_base::Error,
        models::{api_config::ApiConfig, api_configuration_old},
    },
    users::users_api,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error<users_api::GetUsersMeError>> {
    dotenv::dotenv().expect("Failed to read .env file");

    let developer_token = env::var("DEVELOPER_TOKEN").expect("DEVELOPER_TOKEN not set");

    let api_config = ApiConfig::new();

    // let mut api_config = ApiConfig::new();
    // api_config.set_base_api_url("https://127.0.0.1".to_owned());

    let mut client_config = api_configuration_old::Configuration::new();
    client_config.base_path = api_config.base_api_url();
    client_config.oauth_access_token = Some(developer_token.clone());

    let params = users_api::GetUsersMeParams::default();

    let user = users_api::get_users_me(&client_config, params).await?;

    // let user = match resp {
    //     Ok(resp) => resp,
    //     Err(err) => {
    //         println!("Error: {:#?}", err);
    //         return;
    //     }
    // };

    println!("Current user:\n{user:#?}\n");
    Ok(())
}
