use url::Url;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UrlParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
}

pub fn request_process(server: tiny_http::Server) -> Result<(String, String), String> {
    match server.recv() {
        Ok(rq) => {
            let base_url = Url::parse("http://127.0.0.1").expect("Error parsing base URL");

            let url = base_url
                .join(rq.url())
                .expect("Error parsing URL from request");

            let query_params: UrlParams = match serde_qs::from_str(url.query().unwrap_or_default())
            {
                Ok(query_params) => query_params,
                Err(_) => return Err("Error srializing url query".to_string()),
            };

            match query_params.code {
                Some(code) => {
                    let response = tiny_http::Response::empty(200);
                    rq.respond(response)
                        .expect("Error sending response local server");
                    Ok((code, query_params.state.unwrap_or_default()))
                }
                None => Err(format!(
                    "{} {}",
                    query_params.error.unwrap_or_default(),
                    query_params.error_description.unwrap_or_default()
                )),
            }
        }
        Err(_) => Err("Error receiving request from local server".to_string()),
    }
}
