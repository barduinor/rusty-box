/// Users API tests
use pretty_assertions::assert_eq;
use rusty_box::rest_api::users::models::post_users_request;
use rusty_box::rest_api::users::models::user_full;
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
    assert!(users_list.limit.is_some());

    // there must be at least one user
    let user_0 = &users_list.entries.unwrap()[0];
    assert!(user_0.id.is_some());
    assert!(user_0.name.is_some());
    assert!(user_0.login.is_some());
    assert_eq!(
        user_0.r#type,
        rusty_box::rest_api::users::models::user::RHashType::User
    );
    // assert_eq!(user_0.name.as_ref().unwrap(), "Box Admin"); // this will fail

    Ok(())
}

#[tokio::test]
async fn users_get_by_id() -> Result<(), AuthError> {
    let mut client = common::box_client::get_box_client()?;

    let me = users_api::me(&mut client, None).await?;

    let user_id = me.id.unwrap();

    let fields = vec![
        "id".to_string(),
        "type".to_string(),
        "name".to_string(),
        "login".to_string(),
    ];

    let user = users_api::id(&mut client, &user_id, Some(fields)).await?;

    assert!(user.id.is_some());
    assert!(user.name.is_some());
    assert!(user.login.is_some());
    assert_eq!(
        user.r#type,
        rusty_box::rest_api::users::models::user_full::RHashType::User
    );

    Ok(())
}

#[tokio::test]
async fn users_create() -> Result<(), AuthError> {
    let mut client = common::box_client::get_box_client()?;

    let new_user_request = post_users_request::PostUsersRequest {
        name: "Test User".to_string(),
        login: Some("test.user@gmail.local".to_string()),
        is_platform_access_only: Some(false),
        role: Some(post_users_request::Role::Coadmin),
        language: Some("en".to_string()),
        is_sync_enabled: Some(true),
        job_title: Some("Test Job Title".to_string()),
        phone: Some("555-555-5555".to_string()),
        address: Some("123 Test St".to_string()),
        space_amount: Some(1073741824),
        // tracking_codes: Some(vec!["test-tracking-code".to_string()]),
        can_see_managed_users: Some(true),
        timezone: Some("America/Los_Angeles".to_string()),
        is_external_collab_restricted: Some(false),
        is_exempt_from_device_limits: Some(false),
        is_exempt_from_login_verification: Some(false),
        status: Some(post_users_request::Status::Active),
        external_app_user_id: Some("test-external-app-user-id".to_string()),

        ..Default::default()
    };

    let new_user = users_api::create(&mut client, new_user_request).await?;

    // log::info!("new_user: {:#?}", new_user);

    assert!(new_user.id.is_some());
    assert_eq!(new_user.name.unwrap(), "Test User");
    assert_eq!(new_user.login.unwrap(), "test.user@gmail.local");
    assert_eq!(new_user.language.unwrap(), "en");
    assert_eq!(new_user.job_title.unwrap(), "Test Job Title");
    assert_eq!(new_user.phone.unwrap(), "555-555-5555");
    assert_eq!(new_user.address.unwrap(), "123 Test St");
    assert_eq!(new_user.space_amount.unwrap(), 1073741824);
    assert_eq!(new_user.timezone.unwrap(), "America/Los_Angeles");
    assert_eq!(new_user.status.unwrap(), user_full::Status::Active);

    // fields not included by default
    // assert_eq!(new_user.role.unwrap(), user_full::Role::Coadmin); // not normaly returned
    // assert_eq!(new_user.is_sync_enabled.unwrap(), true);
    // assert_eq!(new_user.tracking_codes.unwrap(), vec!["test-tracking-code".to_string()]);
    // assert_eq!(new_user.can_see_managed_users.unwrap(), true);
    // assert_eq!(new_user.is_external_collab_restricted.unwrap(), false);
    // assert_eq!(new_user.is_exempt_from_device_limits.unwrap(), false);
    // assert_eq!(new_user.is_exempt_from_login_verification.unwrap(), false);
    // assert_eq!(
    //     new_user.external_app_user_id.unwrap(),
    //     "test-external-app-user-id"
    // );

    Ok(())
}
