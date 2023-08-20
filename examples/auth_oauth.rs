mod oauth;

use rusty_box::{BoxAPIError, BoxClient, Config, OAuth};

use crate::oauth::{authorize_app, storage};
use std::env;

#[tokio::main]
async fn main() -> Result<(), BoxAPIError> {
    dotenv::from_filename(".oauth.env").expect("Failed to read .env file");

    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");
    let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI not set");

    let config = Config::new();
    let oauth = OAuth::new(
        config,
        client_id,
        client_secret,
        Some(storage::save_access_token),
    );

    // Load the OAuth token from the cache file
    let oauth_json = oauth::storage::load_access_token();

    let oauth = match oauth_json {
        Ok(oauth_json) => {
            println!("Cached token found, refreshing");
            let oauth: OAuth =
                serde_json::from_str(&oauth_json).expect("Failed to parse cached token");
            oauth
        }
        Err(_) => {
            println!("No cached token found, authorizing app");
            authorize_app::authorize_app(oauth, Some(redirect_uri)).await?
        }
    };

    let mut client = BoxClient::new(Box::new(oauth));

    let me = rusty_box::users_api::me(&mut client, None).await;
    println!("Me:\n{me:#?}\n");

    Ok(())
}
