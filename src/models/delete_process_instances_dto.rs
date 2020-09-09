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
pub struct DeleteProcessInstancesDto {
    /// A list process instance ids to delete.
    #[serde(rename = "processInstanceIds", skip_serializing_if = "Option::is_none")]
    pub process_instance_ids: Option<Vec<String>>,
    /// A string with delete reason.
    #[serde(rename = "deleteReason", skip_serializing_if = "Option::is_none")]
    pub delete_reason: Option<String>,
    /// Skip execution listener invocation for activities that are started or ended as part of this request.
    #[serde(rename = "skipCustomListeners", skip_serializing_if = "Option::is_none")]
    pub skip_custom_listeners: Option<bool>,
    /// Skip deletion of the subprocesses related to deleted processes as part of this request.
    #[serde(rename = "skipSubprocesses", skip_serializing_if = "Option::is_none")]
    pub skip_subprocesses: Option<bool>,
    #[serde(rename = "processInstanceQuery", skip_serializing_if = "Option::is_none")]
    pub process_instance_query: Option<crate::models::ProcessInstanceQueryDto>,
    #[serde(rename = "historicProcessInstanceQuery", skip_serializing_if = "Option::is_none")]
    pub historic_process_instance_query: Option<crate::models::HistoricProcessInstanceQueryDto>,
}

impl DeleteProcessInstancesDto {
    pub fn new() -> DeleteProcessInstancesDto {
        DeleteProcessInstancesDto {
            process_instance_ids: None,
            delete_reason: None,
            skip_custom_listeners: None,
            skip_subprocesses: None,
            process_instance_query: None,
            historic_process_instance_query: None,
        }
    }
}


