//! Method, error and parameter types for the Meta endpoint.
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

pub struct Meta<'api> {
    auth: &'api Auth
}

pub fn new(auth: &Auth) -> Meta {
    Meta { auth }
}

/// Errors for the [Get GitHub meta information](Meta::get_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum MetaGetError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Not Modified")]
    Status304,
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

/// Errors for the [Get Octocat](Meta::get_octocat_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum MetaGetOctocatError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

/// Errors for the [Get the Zen of GitHub](Meta::get_zen_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum MetaGetZenError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

/// Errors for the [GitHub API Root](Meta::root_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum MetaRootError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Status code: {}", code)]
    Generic { code: u16 },
}


/// Query parameters for the [Get Octocat](Meta::get_octocat_async()) endpoint.
#[derive(Default, Serialize)]
pub struct MetaGetOctocatParams<'req> {
    /// The words to show in Octocat's speech bubble
    s: Option<&'req str>
}

impl<'req> MetaGetOctocatParams<'req> {
    pub fn new() -> Self {
        Self::default()
    }

    /// The words to show in Octocat's speech bubble
    pub fn s(self, s: &'req str) -> Self {
        Self { 
            s: Some(s),
        }
    }
}


impl<'api> Meta<'api> {
    /// ---
    ///
    /// # Get GitHub meta information
    ///
    /// Returns meta information about GitHub, including a list of GitHub's IP addresses. For more information, see "[About GitHub's IP addresses](https://help.github.com/articles/about-github-s-ip-addresses/)."
    /// 
    /// **Note:** The IP addresses shown in the documentation's response are only example values. You must always query the API directly to get the latest list of IP addresses.
    /// 
    /// [GitHub API docs for get](https://docs.github.com/rest/reference/meta/#get-github-meta-information)    
    /// ---
    pub async fn get_async(&self) -> Result<ApiOverview, MetaGetError> {

        let request_uri = format!("{}/meta", super::GITHUB_BASE_API_URL);


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
                304 => Err(MetaGetError::Status304),
                code => Err(MetaGetError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get GitHub meta information
    ///
    /// Returns meta information about GitHub, including a list of GitHub's IP addresses. For more information, see "[About GitHub's IP addresses](https://help.github.com/articles/about-github-s-ip-addresses/)."
    /// 
    /// **Note:** The IP addresses shown in the documentation's response are only example values. You must always query the API directly to get the latest list of IP addresses.
    /// 
    /// [GitHub API docs for get](https://docs.github.com/rest/reference/meta/#get-github-meta-information)    
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get(&self) -> Result<ApiOverview, MetaGetError> {

        let request_uri = format!("{}/meta", super::GITHUB_BASE_API_URL);


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
                304 => Err(MetaGetError::Status304),
                code => Err(MetaGetError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get Octocat
    ///
    /// Get the octocat as ASCII art
        
    /// ---
    pub async fn get_octocat_async(&self, query_params: Option<impl Into<MetaGetOctocatParams<'api>>>) -> Result<String, MetaGetOctocatError> {

        let mut request_uri = format!("{}/octocat", super::GITHUB_BASE_API_URL);

        if let Some(params) = query_params {
            request_uri.push_str("?");
            request_uri.push_str(&serde_urlencoded::to_string(params.into())?);
        }

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
                code => Err(MetaGetOctocatError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get Octocat
    ///
    /// Get the octocat as ASCII art
        
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_octocat(&self, query_params: Option<impl Into<MetaGetOctocatParams<'api>>>) -> Result<String, MetaGetOctocatError> {

        let mut request_uri = format!("{}/octocat", super::GITHUB_BASE_API_URL);

        if let Some(params) = query_params {
            request_uri.push_str("?");
            let qp: MetaGetOctocatParams = params.into();
            request_uri.push_str(&serde_urlencoded::to_string(qp)?);
        }

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
                code => Err(MetaGetOctocatError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get the Zen of GitHub
    ///
    /// Get a random sentence from the Zen of GitHub
        
    /// ---
    pub async fn get_zen_async(&self) -> Result<String, MetaGetZenError> {

        let request_uri = format!("{}/zen", super::GITHUB_BASE_API_URL);


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
                code => Err(MetaGetZenError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get the Zen of GitHub
    ///
    /// Get a random sentence from the Zen of GitHub
        
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_zen(&self) -> Result<String, MetaGetZenError> {

        let request_uri = format!("{}/zen", super::GITHUB_BASE_API_URL);


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
                code => Err(MetaGetZenError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # GitHub API Root
    ///
    /// Get Hypermedia links to resources accessible in GitHub's REST API
        
    /// ---
    pub async fn root_async(&self) -> Result<GetMetaRootResponse200, MetaRootError> {

        let request_uri = format!("{}/", super::GITHUB_BASE_API_URL);


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
                code => Err(MetaRootError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # GitHub API Root
    ///
    /// Get Hypermedia links to resources accessible in GitHub's REST API
        
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn root(&self) -> Result<GetMetaRootResponse200, MetaRootError> {

        let request_uri = format!("{}/", super::GITHUB_BASE_API_URL);


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
                code => Err(MetaRootError::Generic { code }),
            }
        }
    }

}
