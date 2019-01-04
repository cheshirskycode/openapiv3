use crate::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    /// REQUIRED. The title of the application.
    pub title: String,
    /// A short description of the application.
    /// CommonMark syntax MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A URL to the Terms of Service for the API.
    /// MUST be in the format of a URL.
    #[serde(rename = "termsOfService", skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,
    /// The contact information for the exposed API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    /// The license information for the exposed API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
    /// REQUIRED. The version of the OpenAPI document (which is distinct from
    /// the OpenAPI Specification version or the API implementation version).
    pub version: String,
}