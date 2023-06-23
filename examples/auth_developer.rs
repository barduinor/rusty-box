// use cargo run --example users_main to run this file
// use dotenv;

use rusty_box::{
    auth::auth_developer::DeveloperToken,
    box_client::BoxClient,
    config::Config,
    rest_api::{api::api_base::Error, users::users_api},
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error<users_api::GetUsersMeError>> {
    dotenv::from_filename(".dev.env").ok();

    let config = Config::new();
    let auth = DeveloperToken::new(
        config,
        env::var("DEVELOPER_TOKEN").expect("DEVELOPER_TOKEN must be set"),
    );

    let mut client = BoxClient::new(Box::new(auth.clone()));

    let fields = vec![
        // "id".to_string(),
        // "type".to_string(),
        // "name".to_string(),
        // "login".to_string(),
    ];

    let me = users_api::me(&mut client, Some(fields.clone())).await;
    println!("Me:\n{me:#?}\n");

    // let client = BoxClient::new(Box::new(auth.clone()));

    let params = users_api::GetUsersParams {
        fields: Some(fields),
        ..Default::default()
    };
    let result = users_api::list(&mut client, params).await;
    // println!("Users: {result:#?}\n");
    print!("Users:\n");

    let user_list = match result {
        Ok(users) => users,
        Err(e) => {
            println!("Error: {:#?}", e);
            return Ok(());
        }
    };

    if let Some(users) = user_list.entries {
        for user in users {
            println!(
                "{}\t{}\t{}",
                user.id.unwrap(),
                user.name.unwrap(),
                user.login.unwrap()
            );
        }
    }

    Ok(())
}
