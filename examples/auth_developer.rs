// use cargo run --example users_main to run this file
// use dotenv;

use rusty_box::{
    auth::auth_developer::DeveloperToken,
    box_client::BoxClient,
    config::Config,
    rest_api::{
        api::{
            api_base::Error,
            models::{api_config::ApiConfig, api_configuration_old},
        },
        users::users_api,
    },
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error<users_api::GetUsersMeError>> {
    dotenv::from_filename(".dev.env").ok();

    let config = Config::new();
    let auth = DeveloperToken::new(
        config,
        env::var("DEVELOPER_TOKEN").expect("DEVELOPER_TOKEN must be set"),
    );

    // todo!("re implement");

    // let mut client = BoxClient::new(Box::new(auth.clone()));
    // let access_token = client.auth.access_token().await.unwrap_or_default();

    // let api_config = ApiConfig::new();

    // let mut client_config = api_configuration_old::Configuration::new();
    // client_config.base_path = api_config.base_api_url();
    // client_config.oauth_access_token = Some(access_token);

    // let params = users_api::GetUsersMeParams::default();

    // let user = users_api::get_users_me(&client_config, params).await?;

    // println!("Current user:\n{user:#?}\n");
    Ok(())
}
