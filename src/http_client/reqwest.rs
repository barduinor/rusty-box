//! The client implementation for the reqwest HTTP client, which is async by
//! default.

use crate::rest_api::errors::{error_api::BoxAPIError, model::client_error::BoxAPIErrorResponse};

use super::{BaseHttpClient, Form, Headers, Query};

use std::convert::TryInto;
use std::time::Duration;

use async_trait::async_trait;
use reqwest::{Method, RequestBuilder};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ReqwestClient {
    /// reqwest needs an instance of its client to perform requests.
    client: reqwest::Client,
}

impl Default for ReqwestClient {
    fn default() -> Self {
        let client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(10))
            .build()
            // building with these options cannot fail
            .unwrap();
        Self { client }
    }
}

impl ReqwestClient {
    async fn request<D>(
        &self,
        method: Method,
        url: &str,
        headers: Option<&Headers>,
        add_data: D,
    ) -> Result<String, BoxAPIError>
    where
        D: Fn(RequestBuilder) -> RequestBuilder,
    {
        let mut request = self.client.request(method.clone(), url);

        // Setting the headers, if any
        if let Some(headers) = headers {
            // The headers need to be converted into a `reqwest::HeaderMap`,
            // which won't fail as long as its contents are ASCII. This is an
            // internal function, so the condition cannot be broken by the user
            // and will always be true.
            //
            // The content-type header will be set automatically.
            let headers = headers.try_into().unwrap();

            request = request.headers(headers);
        }

        // Configuring the request for the specific type (get/post/put/delete)
        request = add_data(request);

        // Finally performing the request and handling the response
        log::info!("Making request {:?}", request);
        let response = request.send().await?;
        let status = response.status();
        let resp_text = response.text().await?;

        // Making sure that the status code is OK
        if status.is_success() {
            Ok(resp_text)
        } else {
            let resp_error = serde_json::from_str::<BoxAPIErrorResponse>(&resp_text)?;
            Err(BoxAPIError::ResponseError(resp_error))
        }
    }
}

#[async_trait]
impl BaseHttpClient for ReqwestClient {
    type Error = BoxAPIError;

    #[inline]
    async fn get(
        &self,
        url: &str,
        headers: Option<&Headers>,
        query: &Query,
    ) -> Result<String, Self::Error> {
        self.request(Method::GET, url, headers, |req| req.query(query))
            .await
    }

    #[inline]
    async fn post(
        &self,
        url: &str,
        headers: Option<&Headers>,
        query: Option<&Query>,
        payload: &Value,
    ) -> Result<String, Self::Error> {
        self.request(Method::POST, url, headers, |req| {
            req.query(&query).json(payload)
        })
        .await
    }

    #[inline]
    async fn post_form(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Form<'_>,
    ) -> Result<String, Self::Error> {
        self.request(Method::POST, url, headers, |req| req.form(payload))
            .await
    }

    #[inline]
    async fn put(
        &self,
        url: &str,
        headers: Option<&Headers>,
        query: Option<&Query>,
        payload: &Value,
    ) -> Result<String, Self::Error> {
        self.request(Method::PUT, url, headers, |req| {
            req.query(&query).json(payload)
        })
        .await
    }

    #[inline]
    async fn delete(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<String, Self::Error> {
        self.request(Method::DELETE, url, headers, |req| req.json(payload))
            .await
    }

    // TODO: implement method OPTION for reqwest
}
