use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                },
                serde_json::Value::String(s) => params.push((format!("{}[{}]", prefix, key), s.clone())),
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

pub mod authorization_api;
pub mod box_sign_api;
pub mod classifications_api;
pub mod classifications_on_files_api;
pub mod classifications_on_folders_api;
pub mod collaborations_api;
pub mod collaborations_list_api;
pub mod collections_api;
pub mod comments_api;
pub mod device_pinners_api;
pub mod domain_restrictions_for_collaborations_api;
pub mod domain_restrictions_user_exemptions_api;
pub mod downloads_api;
pub mod email_aliases_api;
pub mod events_api;
pub mod file_requests_api;
pub mod file_version_legal_holds_api;
pub mod file_version_retentions_api;
pub mod file_versions_api;
pub mod files_api;
pub mod folder_locks_api;
pub mod folders_api;
pub mod group_memberships_api;
pub mod groups_api;
pub mod integration_mappings_slack_api;
pub mod invites_api;
pub mod legal_hold_policies_api;
pub mod legal_hold_policy_assignments_api;
pub mod metadata_cascade_policies_api;
pub mod metadata_instances_files_api;
pub mod metadata_instances_folders_api;
pub mod metadata_templates_api;
pub mod post_integration_mappings_slack_api;
pub mod recent_items_api;
pub mod retention_policies_api;
pub mod retention_policy_assignments_api;
pub mod search_api;
pub mod shared_links_files_api;
pub mod shared_links_folders_api;
pub mod shared_links_web_links_api;
pub mod shield_information_barrier_reports_api;
pub mod shield_information_barrier_segment_members_api;
pub mod shield_information_barrier_segment_restrictions_api;
pub mod shield_information_barrier_segments_api;
pub mod shield_information_barriers_api;
pub mod skills_api;
pub mod storage_policies_api;
pub mod storage_policy_assignments_api;
pub mod task_assignments_api;
pub mod tasks_api;
pub mod templates_api;
pub mod terms_of_service_api;
pub mod terms_of_service_user_statuses_api;
pub mod transfer_folders_api;
pub mod trashed_files_api;
pub mod trashed_folders_api;
pub mod trashed_items_api;
pub mod trashed_web_links_api;
pub mod uploads_api;
pub mod uploads_chunked_api;
pub mod user_avatars_api;
pub mod users_api;
pub mod watermarks_files_api;
pub mod watermarks_folders_api;
pub mod web_links_api;
pub mod webhooks_api;
pub mod workflows_api;
pub mod zip_downloads_api;

pub mod configuration;
