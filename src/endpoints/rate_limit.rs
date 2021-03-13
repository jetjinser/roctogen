//! Method, error and parameter types for the RateLimit endpoint.
#![allow(
    unused_imports,
)]
/* 
 * GitHub v3 REST API
 *
 * GitHub's v3 REST API.
 *
 * OpenAPI spec version: 1.1.4
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde::Deserialize;

use crate::adapters::{AdapterError, FromJson, GitHubRequest, GitHubRequestBuilder, GitHubResponseExt, ToJson};
use crate::auth::Auth;
use crate::models::*;

use super::PerPage;

use std::collections::HashMap;
use serde_json::value::Value;

pub struct RateLimit<'api> {
    auth: &'api Auth
}

pub fn new(auth: &Auth) -> RateLimit {
    RateLimit { auth }
}

/// Errors for the [Get rate limit status for the authenticated user](RateLimit::get_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum RateLimitGetError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Not Modified")]
    Status304,
    #[error("Resource Not Found")]
    Status404(BasicError),
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}



impl<'api> RateLimit<'api> {
    /// ---
    ///
    /// # Get rate limit status for the authenticated user
    ///
    /// **Note:** Accessing this endpoint does not count against your REST API rate limit.
    /// 
    /// **Note:** The `rate` object is deprecated. If you're writing new API client code or updating existing code, you should use the `core` object instead of the `rate` object. The `core` object contains the same information that is present in the `rate` object.
    /// 
    /// [GitHub API docs for get](https://docs.github.com/rest/reference/rate_limit/#get-rate-limit-status-for-the-authenticated-user)    
    /// ---
    pub async fn get_async(&self) -> Result<RateLimitOverview, RateLimitGetError> {

        let request_uri = format!("{}/rate_limit", super::GITHUB_BASE_API_URL);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(github_response.to_json()?)
        } else {
            match github_response.status_code() {
                304 => Err(RateLimitGetError::Status304),
                404 => Err(RateLimitGetError::Status404(github_response.to_json()?)),
                code => Err(RateLimitGetError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get rate limit status for the authenticated user
    ///
    /// **Note:** Accessing this endpoint does not count against your REST API rate limit.
    /// 
    /// **Note:** The `rate` object is deprecated. If you're writing new API client code or updating existing code, you should use the `core` object instead of the `rate` object. The `core` object contains the same information that is present in the `rate` object.
    /// 
    /// [GitHub API docs for get](https://docs.github.com/rest/reference/rate_limit/#get-rate-limit-status-for-the-authenticated-user)    
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get(&self) -> Result<RateLimitOverview, RateLimitGetError> {

        let request_uri = format!("{}/rate_limit", super::GITHUB_BASE_API_URL);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(github_response.to_json()?)
        } else {
            match github_response.status_code() {
                304 => Err(RateLimitGetError::Status304),
                404 => Err(RateLimitGetError::Status404(github_response.to_json()?)),
                code => Err(RateLimitGetError::Generic { code }),
            }
        }
    }

}
