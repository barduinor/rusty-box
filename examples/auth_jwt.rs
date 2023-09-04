// use cargo run --example users_main to run this file
// use dotenv;

use rusty_box::{
    auth::auth_jwt::JWTAuth,
    auth::auth_jwt::SubjectType,
    client::{box_client::BoxClient, client_error::BoxAPIError},
    config::Config,
    rest_api::users::users_api,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), BoxAPIError> {
    dotenv::from_filename(".jwt.env").ok();

    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");

    let env_subject_type = env::var("BOX_SUBJECT_TYPE").expect("BOX_SUBJECT_TYPE must be set");
    let box_subject_type = match env_subject_type.as_str() {
        "user" => SubjectType::User,
        "enterprise" => SubjectType::Enterprise,
        _ => panic!("BOX_SUBJECT_TYPE must be either 'user' or 'enterprise'"),
    };

    let box_subject_id = env::var("BOX_SUBJECT_ID").expect("BOX_SUBJECT_ID must be set");

    let public_key_id = env::var("PUBLIC_KEY_ID").expect("PUBLIC_KEY_ID must be set");

    let encrypted_private_key =
        env::var("ENCRYPTED_PRIVATE_KEY").expect("ENCRYPTED_PRIVATE_KEY must be set");

    let passphrase = env::var("PASSPHRASE").expect("PASSPHRASE must be set");

    let config = Config::new();
    let auth = JWTAuth::new(
        config,
        client_id,
        client_secret,
        box_subject_type,
        box_subject_id,
        public_key_id,
        encrypted_private_key,
        passphrase,
    );

    let mut client = BoxClient::new(Box::new(auth));

    let fields = vec![];

    let me = users_api::me(&mut client, Some(fields)).await?;
    println!("Me:\n{me:#?}\n");

    Ok(())
}
