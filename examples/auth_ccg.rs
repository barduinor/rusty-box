// use cargo run --example users_main to run this file
// use dotenv;

use rusty_box::{
    auth::{
        ccg::{CCGAuth, SubjectType},
        Auth,
    },
    config::Config,
    rest_api::{
        api::{api_base::Error, models::api_configuration_old},
        users::users_api,
    },
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
    let mut ccg_auth = CCGAuth::new(
        config,
        client_id,
        client_secret,
        box_subject_type,
        box_subject_id,
    );
    // let access_token = ccg_auth.fetch_access_token().await.unwrap_or_default();
    let access_token = ccg_auth.access_token().await.unwrap_or_default();

    let mut client_config = api_configuration_old::Configuration::new();
    client_config.base_path = ccg_auth.config.base_api_url();
    client_config.oauth_access_token = Some(access_token);

    // let paramsx = users_api::GetUsersMeParams::default();
    let params = users_api::GetUsersMeParams::default();

    let user = users_api::get_users_me(&client_config, params).await?;

    println!("Current user:\n{user:#?}\n");
    Ok(())
}
