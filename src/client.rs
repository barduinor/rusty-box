use crate::{auth::Auth, rest_api::api::models::api_config::ApiConfig};

pub struct Client {
    config: ApiConfig,
    auth: Auth,
}
