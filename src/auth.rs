use async_trait::async_trait;

pub mod auth_ccg;
pub mod auth_developer;

#[async_trait]
pub trait Auth {
    type Error;

    async fn access_token(&mut self) -> Result<String, Self::Error>;
    // fn store_auth(&self);
    fn to_json(&self) -> Result<String, Self::Error>;
}
