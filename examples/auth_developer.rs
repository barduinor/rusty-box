// use cargo run --example users_main to run this file
// use dotenv;

use rusty_box::{
    auth::auth_developer::DevAuth,
    client::{box_client::BoxClient, client_error::BoxAPIError},
    config::Config,
    rest_api::users::users_api,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), BoxAPIError> {
    dotenv::from_filename(".dev.env").ok();

    let config = Config::new();
    let auth = DevAuth::new(
        config,
        env::var("DEVELOPER_TOKEN").expect("DEVELOPER_TOKEN must be set"),
    );

    let mut client = BoxClient::new(Box::new(auth.clone()));

    let fields = vec![];

    let me = users_api::me(&mut client, Some(fields)).await?;
    println!("Me:\n{me:#?}\n");

    Ok(())
}
