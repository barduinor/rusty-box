/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

use crate::rest_api::api::models::tracking_code::TrackingCode;

use super::put_users_id_request_notification_email::PutUsersIdRequestNotificationEmail;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PutUsersIdRequest {
    /// Set this to `null` to roll the user out of the enterprise and make them a free user
    #[serde(
        rename = "enterprise",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub enterprise: Option<Option<String>>,
    /// Whether the user should receive an email when they are rolled out of an enterprise
    #[serde(rename = "notify", skip_serializing_if = "Option::is_none")]
    pub notify: Option<bool>,
    /// The name of the user
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The email address the user uses to log in  Note: If the target user's email is not confirmed, then the primary login address cannot be changed.
    #[serde(rename = "login", skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    /// The user’s enterprise role
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    /// The language of the user, formatted in modified version of the [ISO 639-1](/guides/api-calls/language-codes) format.
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Whether the user can use Box Sync
    #[serde(rename = "is_sync_enabled", skip_serializing_if = "Option::is_none")]
    pub is_sync_enabled: Option<bool>,
    /// The user’s job title
    #[serde(rename = "job_title", skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    /// The user’s phone number
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The user’s address
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Tracking codes allow an admin to generate reports from the admin console and assign an attribute to a specific group of users. This setting must be enabled for an enterprise before it can be used.
    #[serde(rename = "tracking_codes", skip_serializing_if = "Option::is_none")]
    pub tracking_codes: Option<Vec<TrackingCode>>,
    /// Whether the user can see other enterprise users in their contact list
    #[serde(
        rename = "can_see_managed_users",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_see_managed_users: Option<bool>,
    /// The user's timezone
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Whether the user is allowed to collaborate with users outside their enterprise
    #[serde(
        rename = "is_external_collab_restricted",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_external_collab_restricted: Option<bool>,
    /// Whether to exempt the user from enterprise device limits
    #[serde(
        rename = "is_exempt_from_device_limits",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_exempt_from_device_limits: Option<bool>,
    /// Whether the user must use two-factor authentication
    #[serde(
        rename = "is_exempt_from_login_verification",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_exempt_from_login_verification: Option<bool>,
    /// Whether the user is required to reset their password
    #[serde(
        rename = "is_password_reset_required",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_password_reset_required: Option<bool>,
    /// The user's account status
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The user’s total available space in bytes. Set this to `-1` to indicate unlimited storage.
    #[serde(rename = "space_amount", skip_serializing_if = "Option::is_none")]
    pub space_amount: Option<i64>,
    #[serde(
        rename = "notification_email",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_email: Option<Option<Box<PutUsersIdRequestNotificationEmail>>>,
    /// An external identifier for an app user, which can be used to look up the user. This can be used to tie user IDs from external identity providers to Box users.  Note: In order to update this field, you need to request a token using the application that created the app user.
    #[serde(
        rename = "external_app_user_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_app_user_id: Option<String>,
}

impl PutUsersIdRequest {
    pub fn new() -> PutUsersIdRequest {
        PutUsersIdRequest {
            enterprise: None,
            notify: None,
            name: None,
            login: None,
            role: None,
            language: None,
            is_sync_enabled: None,
            job_title: None,
            phone: None,
            address: None,
            tracking_codes: None,
            can_see_managed_users: None,
            timezone: None,
            is_external_collab_restricted: None,
            is_exempt_from_device_limits: None,
            is_exempt_from_login_verification: None,
            is_password_reset_required: None,
            status: None,
            space_amount: None,
            notification_email: None,
            external_app_user_id: None,
        }
    }
}

/// The user’s enterprise role
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "coadmin")]
    Coadmin,
    #[serde(rename = "user")]
    User,
}

impl Default for Role {
    fn default() -> Role {
        Self::Coadmin
    }
}
/// The user's account status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "cannot_delete_edit")]
    CannotDeleteEdit,
    #[serde(rename = "cannot_delete_edit_upload")]
    CannotDeleteEditUpload,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}