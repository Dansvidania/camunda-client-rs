/*
 * Camunda BPM REST API
 *
 * OpenApi Spec for Camunda BPM REST API.
 *
 * The version of the OpenAPI document: 7.13.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationExceptionDto {
    /// The id of the user that does not have expected permissions
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "missingAuthorizations", skip_serializing_if = "Option::is_none")]
    pub missing_authorizations: Option<Vec<crate::models::MissingAuthorizationDto>>,
    /// An exception class indicating the occurred error.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// A detailed message of the error.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl AuthorizationExceptionDto {
    pub fn new() -> AuthorizationExceptionDto {
        AuthorizationExceptionDto {
            user_id: None,
            missing_authorizations: None,
            _type: None,
            message: None,
        }
    }
}


