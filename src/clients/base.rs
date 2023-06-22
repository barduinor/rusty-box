use std::fmt;

// use chrono::Utc;
use maybe_async::maybe_async;
// use serde_json::Value;
// use serde_json::Value;

// use crate::http_client::Query;

// use super::ClientResult;
// use crate::htt

// use super::ClientResult;

/// This trait implements the basic endpoints from the Spotify API that may be
/// accessed without user authorization, including parts of the authentication
/// flow that are shared, and the endpoints.
#[maybe_async]
pub trait BaseClient
where
    Self: Send + Sync + Default + Clone + fmt::Debug,
{
    // fn get_config(&self) -> &Config;
    // fn get_http(&self) -> &HttpClient;
    // fn get_creds(&self) -> &Credentials;

    // Note that the token is wrapped by a `Mutex` in order to allow interior
    // mutability. This is required so that the entire client doesn't have to
    // be mutable (the token is accessed to from every endpoint).
    // fn get_token(&self) -> Arc<Mutex<Option<Token>>>;

    // Returns the absolute URL for an endpoint in the API.
    // fn api_url(&self, url: &str) -> String {
    //     let base_api_url = self.auth.base_api_url();
    //     let mut base = self.get_config().api_base_url.clone();
    //     if !base.ends_with('/') {
    //         base.push('/');
    //     }
    //     base + url
    // }

    // /// Returns the absolute URL for an authentication step in the API.
    // fn auth_url(&self, url: &str) -> String {
    //     let mut base = self.get_config().auth_base_url.clone();
    //     if !base.ends_with('/') {
    //         base.push('/');
    //     }
    //     base + url
    // }

    // /// Refetch the current access token given a refresh token.
    // async fn refetch_token(&self) -> ClientResult<Option<Token>>;

    // /// Re-authenticate the client automatically if it's configured to do so,
    // /// which uses the refresh token to obtain a new access token.
    // async fn auto_reauth(&self) -> ClientResult<()> {
    //     if !self.get_config().token_refreshing {
    //         return Ok(());
    //     }

    //     // NOTE: It's important to not leave the token locked, or else a
    //     // deadlock when calling `refresh_token` will occur.
    //     let should_reauth = self
    //         .get_token()
    //         .lock()
    //         .await
    //         .unwrap()
    //         .as_ref()
    //         .map_or(false, Token::is_expired);

    //     if should_reauth {
    //         self.refresh_token().await
    //     } else {
    //         Ok(())
    //     }
    // }

    // /// Refreshes the current access token given a refresh token. The obtained
    // /// token will be saved internally.
    // async fn refresh_token(&self) -> ClientResult<()> {
    //     let token = self.refetch_token().await?;
    //     *self.get_token().lock().await.unwrap() = token;
    //     self.write_token_cache().await
    // }

    // /// The headers required for authenticated requests to the API.
    // ///
    // /// Since this is accessed by authenticated requests always, it's where the
    // /// automatic reauthentication takes place, if enabled.
    // #[doc(hidden)]
    // async fn auth_headers(&self) -> Headers {
    //     self.auto_reauth()
    //         .await
    //         .expect("Failed to re-authenticate automatically, please authenticate");

    //     self.get_token()
    //         .lock()
    //         .await
    //         .expect("Failed to acquire lock")
    //         .as_ref()
    //         .expect("RSpotify not authenticated")
    //         .auth_headers()
    // }

    // HTTP-related methods for the Spotify client. They wrap up the basic HTTP
    // client with its specific usage for endpoints or authentication.

    // Convenience method to send GET requests related to an endpoint in the
    // API.
    // #[doc(hidden)]
    // #[inline]
    // async fn api_get(&self, url: &str, payload: &Query<'_>) -> ClientResult<String> {
    //     let url = self.api_url(url);
    //     let headers = self.auth_headers().await;
    //     Ok(self.get_http().get(&url, Some(&headers), payload).await?)
    // }

    // /// Convenience method to send POST requests related to an endpoint in the
    // /// API.
    // #[doc(hidden)]
    // #[inline]
    // async fn api_post(&self, url: &str, payload: &Value) -> ClientResult<String> {
    //     let url = self.api_url(url);
    //     let headers = self.auth_headers().await;
    //     Ok(self.get_http().post(&url, Some(&headers), payload).await?)
    // }

    // /// Convenience method to send PUT requests related to an endpoint in the
    // /// API.
    // #[doc(hidden)]
    // #[inline]
    // async fn api_put(&self, url: &str, payload: &Value) -> ClientResult<String> {
    //     let url = self.api_url(url);
    //     let headers = self.auth_headers().await;
    //     Ok(self.get_http().put(&url, Some(&headers), payload).await?)
    // }

    // /// Convenience method to send DELETE requests related to an endpoint in the
    // /// API.
    // #[doc(hidden)]
    // #[inline]
    // async fn api_delete(&self, url: &str, payload: &Value) -> ClientResult<String> {
    //     let url = self.api_url(url);
    //     let headers = self.auth_headers().await;
    //     Ok(self
    //         .get_http()
    //         .delete(&url, Some(&headers), payload)
    //         .await?)
    // }

    // /// Convenience method to send POST requests related to the authentication
    // /// process.
    // #[doc(hidden)]
    // #[inline]
    // async fn auth_post(
    //     &self,
    //     url: &str,
    //     headers: Option<&Headers>,
    //     payload: &Form<'_>,
    // ) -> ClientResult<String> {
    //     let url = self.auth_url(url);
    //     Ok(self.get_http().post_form(&url, headers, payload).await?)
    // }

    // /// Updates the cache file at the internal cache path.
    // ///
    // /// This should be used whenever it's possible to, even if the cached token
    // /// isn't configured, because this will already check `Config::token_cached`
    // /// and do nothing in that case already.
    // async fn write_token_cache(&self) -> ClientResult<()> {
    //     if !self.get_config().token_cached {
    //         log::info!("Token cache write ignored (not configured)");
    //         return Ok(());
    //     }

    //     log::info!("Writing token cache");
    //     if let Some(tok) = self.get_token().lock().await.unwrap().as_ref() {
    //         tok.write_cache(&self.get_config().cache_path)?;
    //     }

    //     Ok(())
    // }

    // /// Sends a request to Spotify for an access token.
    // async fn fetch_access_token(
    //     &self,
    //     payload: &Form<'_>,
    //     headers: Option<&Headers>,
    // ) -> ClientResult<Token> {
    //     let response = self.auth_post(auth_urls::TOKEN, headers, payload).await?;

    //     let mut tok = serde_json::from_str::<Token>(&response)?;
    //     tok.expires_at = Utc::now().checked_add_signed(tok.expires_in);
    //     Ok(tok)
    // }
}
