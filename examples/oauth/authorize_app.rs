use super::http_request_listener;
use http_request_listener::request_process;
use rusty_box::{AuthError, BoxAPIError, OAuth};

static HOSTNAME: &str = "127.0.0.1";
const PORT: i16 = 5000;

pub async fn authorize_app(
    mut oauth: OAuth,
    redirect_uri: Option<String>,
) -> Result<OAuth, BoxAPIError> {
    let (authorization_url, state_out) = oauth.authorization_url(redirect_uri, None, None)?;

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
    Ok(oauth)
}
