pub mod ccg;
pub mod developer_token;

pub trait Auth {
    type Error;
    fn access_token(&self) -> Result<String, Self::Error>;
    // fn store_auth(&self);
    fn to_json(&self) -> Result<String, Self::Error>;
}
