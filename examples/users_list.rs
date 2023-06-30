use std::env;

use rusty_box::{
    auth::auth_ccg::{CCGAuth, SubjectType},
    client::client::BoxClient,
    config::Config,
    rest_api::{errors::error_api::BoxAPIError, users::users_api},
};

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

    let fields = vec![
        "id".to_string(),
        "type".to_string(),
        "name".to_string(),
        "login".to_string(),
    ];
    let params = users_api::ListUsersParams {
        fields: Some(fields),
        limit: Some(100),
        // filter_term: Some("admin".to_string()),
        ..Default::default()
    };
    let result = users_api::list(&mut client, Some(params)).await?;
    println!("Users:");

    if let Some(users) = result.entries {
        for user in users {
            println!(
                "{}\t{}\t{}\t{}",
                user.id.unwrap(),
                user.r#type.to_string(),
                user.name.unwrap(),
                user.login.unwrap(),
            );
        }
    }

    Ok(())
}
