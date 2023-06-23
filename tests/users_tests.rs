use rusty_box::{self, auth::AuthError, rest_api::users::users_api};
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

    Ok(())
}
