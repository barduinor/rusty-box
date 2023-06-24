/// Users API tests
use pretty_assertions::assert_eq;
use rustybox::{self, auth::AuthError, rest_api::users::users_api};
mod common;

#[tokio::test]
async fn users_me() -> Result<(), AuthError> {
    let mut client = common::box_client::get_box_client()?;

    let me = users_api::me(&mut client, None).await?;
    assert!(me.id.is_some());
    assert!(me.name.is_some());
    assert!(me.login.is_some());

    Ok(())
}

#[tokio::test]
async fn users_list() -> Result<(), AuthError> {
    let mut client = common::box_client::get_box_client()?;

    let fields = vec![
        "id".to_string(),
        "type".to_string(),
        "name".to_string(),
        "login".to_string(),
    ];
    let params = users_api::GetUsersParams {
        fields: Some(fields),
        ..Default::default()
    };

    let users_list = users_api::list(&mut client, Some(params)).await?;

    assert!(users_list.total_count.is_some());
    assert!(users_list.entries.is_some());
    assert!(users_list.limit.is_some());

    // there must be at least one user
    let user_0 = &users_list.entries.unwrap()[0];
    assert!(user_0.id.is_some());
    assert!(user_0.name.is_some());
    assert!(user_0.login.is_some());
    assert_eq!(
        user_0.r#type,
        rustybox::rest_api::users::models::user::RHashType::User
    );
    // assert_eq!(user_0.name.as_ref().unwrap(), "Box Admin"); // this will fail

    Ok(())
}
