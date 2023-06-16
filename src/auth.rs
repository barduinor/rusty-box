use async_trait::async_trait;

pub mod ccg;
pub mod developer_token;

#[async_trait]
pub trait Auth {
    type Error;

    async fn access_token(&mut self) -> Result<String, Self::Error>;
    // fn store_auth(&self);
    fn to_json(&self) -> Result<String, Self::Error>;
}
