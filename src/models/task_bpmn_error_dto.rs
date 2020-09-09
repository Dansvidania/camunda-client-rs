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
pub struct TaskBpmnErrorDto {
    /// An error code that indicates the predefined error. It is used to identify the BPMN error handler.
    #[serde(rename = "errorCode", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// An error message that describes the error.
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// A JSON object containing variable key-value pairs.
    #[serde(rename = "variables", skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, crate::models::VariableValueDto>>,
}

impl TaskBpmnErrorDto {
    pub fn new() -> TaskBpmnErrorDto {
        TaskBpmnErrorDto {
            error_code: None,
            error_message: None,
            variables: None,
        }
    }
}


