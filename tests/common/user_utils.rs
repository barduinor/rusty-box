use rusty_box::{users_api, BoxAPIError, BoxClient};

pub async fn delete_user_by_login(
    client: &mut BoxClient<'_>,
    login: &str,
) -> Result<(), BoxAPIError> {
    let fields = vec![
        "id".to_string(),
        "type".to_string(),
        "name".to_string(),
        "login".to_string(),
    ];

    let params = users_api::ListUsersParams {
        fields: Some(fields),
        filter_term: Some(login.to_string()),
        ..Default::default()
    };

    let users_list = users_api::list(client, Some(params)).await?;

    if let Some(users) = users_list.entries {
        for user in users {
            if let Some(user_id) = user.id {
                users_api::delete(client, &user_id, None, None).await?;
            }
        }
    }

    Ok(())
}
