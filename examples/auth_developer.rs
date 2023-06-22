// use cargo run --example users_main to run this file
// use dotenv;

use rusty_box::{
    auth::auth_developer::DeveloperToken,
    box_client::BoxClient,
    config::Config,
    rest_api::{api::api_base::Error, users::users_api},
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

    let client = BoxClient::new(Box::new(auth.clone()));

    let fields = vec![
        // "id".to_string(),
        // "type".to_string(),
        // "name".to_string(),
        // "login".to_string(),
    ];

    let me = users_api::me(client, Some(fields)).await;
    println!("Me:\n{me:#?}\n");
    Ok(())
}
