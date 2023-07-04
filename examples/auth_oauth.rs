mod oauth;

use std::env;

use oauth::http_request_listener::request_process;
use rusty_box::{AuthError, Config, OAuth};

#[tokio::main]
async fn main() -> Result<(), AuthError> {
    dotenv::from_filename(".oauth.env").expect("Failed to read .env file");

    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");
    let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI not set");

    let hostname = env::var("HOSTNAME").expect("HOSTNAME not set");
    let port = env::var("PORT").expect("PORT not set");

    let config = Config::new();
    let mut oauth = OAuth::new(config, client_id, client_secret, None);

    let (authorization_url, state_out) = oauth.authorization_url(Some(redirect_uri), None, None)?;

    webbrowser::open(&authorization_url).expect("Failed to open browser");

    let hostname_port = hostname + ":" + &port;
    let server = tiny_http::Server::http(hostname_port).unwrap();
    println!("Listening on {}", server.server_addr());

    let (code, state_in) = match request_process(server) {
        Ok((code, state)) => (code, state),
        Err(e) => return Err(AuthError::Generic(e)),
    };
    if state_in != state_out {
        return Err(AuthError::Generic("State mismatch".to_string()));
    }

    let access_token = oauth.request_access_token(code).await?;
    println!("{:#?}", access_token);

    Ok(())
}
