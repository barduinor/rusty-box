// use cargo run --example users_main to run this file
// use dotenv;

use rusty_box::{
    auth::auth_ccg::{CCGAuth, SubjectType},
    client::client::BoxClient,
    config::Config,
    rest_api::{errors::error_api::BoxAPIError, users::users_api},
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), BoxAPIError> {
    dotenv::from_filename(".ccg.env").ok();

    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
    let box_subject_type = SubjectType::Enterprise;
    let box_subject_id = env::var("BOX_ENTERPRISE_ID").expect("BOX_ENTERPRISE_ID must be set");

    let config = Config::new();
    let auth = CCGAuth::new(
        config,
        client_id,
        client_secret,
        box_subject_type,
        box_subject_id,
    );

    let mut client = BoxClient::new(Box::new(auth));

    let fields = vec![];

    let me = users_api::me(&mut client, Some(fields)).await?;
    println!("Me:\n{me:#?}\n");

    Ok(())
}
