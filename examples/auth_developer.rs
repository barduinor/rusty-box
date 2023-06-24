// use cargo run --example users_main to run this file
// use dotenv;

use rustybox::{
    auth::{auth_developer::DeveloperToken, AuthError},
    box_client::BoxClient,
    config::Config,
    rest_api::users::users_api,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), AuthError> {
    dotenv::from_filename(".dev.env").ok();

    let config = Config::new();
    let auth = DeveloperToken::new(
        config,
        env::var("DEVELOPER_TOKEN").expect("DEVELOPER_TOKEN must be set"),
    );

    let mut client = BoxClient::new(Box::new(auth.clone()));

    let fields = vec![];

    let me = users_api::me(&mut client, Some(fields)).await?;
    println!("Me:\n{me:#?}\n");

    Ok(())
}
