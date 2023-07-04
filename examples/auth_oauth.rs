mod oauth;

use std::env;

use oauth::http_request_listener::request_process;
use rusty_box::{AuthError, BoxAPIError, BoxClient, Config, OAuth};

static HOSTNAME: &str = "127.0.0.1";
const PORT: i16 = 5000;

#[tokio::main]
async fn main() -> Result<(), BoxAPIError> {
    dotenv::from_filename(".oauth.env").expect("Failed to read .env file");

    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");
    let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI not set");

    let config = Config::new();
    let mut oauth = OAuth::new(
        config,
        client_id,
        client_secret,
        Some(oauth::storage::save_access_token),
    );

    let (authorization_url, state_out) = oauth.authorization_url(Some(redirect_uri), None, None)?;

    webbrowser::open(&authorization_url).expect("Failed to open browser");

    let hostname_port = HOSTNAME.to_owned() + ":" + &PORT.to_string();
    let server = tiny_http::Server::http(hostname_port).unwrap();
    println!("Listening on {}", server.server_addr());

    let (code, state_in) = match request_process(server) {
        Ok((code, state)) => (code, state),
        Err(e) => return Err(BoxAPIError::AuthError(AuthError::Generic(e))),
    };
    if state_in != state_out {
        return Err(BoxAPIError::AuthError(AuthError::Generic(
            "State mismatch".to_string(),
        )));
    }

    oauth.request_access_token(code).await?;

    let mut client = BoxClient::new(Box::new(oauth));

    let me = rusty_box::users_api::me(&mut client, None).await;
    println!("Me:\n{me:#?}\n");

    Ok(())
}
