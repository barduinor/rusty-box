// use cargo run --example users_main to run this file
// use dotenv;

use rusty_box::{
    auth::auth_ccg::{CCGAuth, SubjectType},
    box_client::BoxClient,
    config::Config,
    rest_api::{api::api_base::Error, users::users_api},
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error<users_api::GetUsersMeError>> {
    dotenv::from_filename(".ccg.env").ok();
    let config = Config::new();

    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
    let box_subject_type = SubjectType::Enterprise;
    let box_subject_id = env::var("BOX_ENTERPRISE_ID").expect("BOX_ENTERPRISE_ID must be set");

    let auth = CCGAuth::new(
        config,
        client_id,
        client_secret,
        box_subject_type,
        box_subject_id,
    );

    let mut client = BoxClient::new(Box::new(auth.clone()));

    // TODO: implement a client

    let access_token = client.auth.access_token().await.unwrap_or_default();
    println!("Access token:\n{:#?}\n", access_token);

    // let me = users_api::me(client).await;
    // println!("Me:\n{me:#?}\n");

    // let mut client_config = api_configuration_old::Configuration::new();
    // client_config.base_path = auth.config.base_api_url();
    // client_config.oauth_access_token = Some(access_token);

    // // let paramsx = users_api::GetUsersMeParams::default();
    // let params = users_api::GetUsersMeParams::default();

    // let user = users_api::get_users_me(&client_config, params).await?;

    // println!("Current user:\n{user:#?}\n");
    Ok(())
}
