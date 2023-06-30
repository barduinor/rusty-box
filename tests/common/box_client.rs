use std::env;

use rusty_box::{AuthError, BoxClient, CCGAuth, Config, SubjectType};

pub fn get_box_client() -> Result<BoxClient<'static>, AuthError> {
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

    let client = BoxClient::new(Box::new(auth));
    Ok(client)
}
