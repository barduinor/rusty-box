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

/// The user’s enterprise role
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "coadmin")]
    Coadmin,
    #[serde(rename = "user")]
    User,
}

impl Default for Role {
    fn default() -> Role {
        Self::Admin
    }
}
