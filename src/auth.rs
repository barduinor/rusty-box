pub mod developer_token;

pub trait Auth {
    type Error;
    fn access_token(&self) -> Result<String, Self::Error>;
}
