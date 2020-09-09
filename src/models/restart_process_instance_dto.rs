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
pub struct RestartProcessInstanceDto {
    /// A list of process instance ids to restart.
    #[serde(rename = "processInstanceIds", skip_serializing_if = "Option::is_none")]
    pub process_instance_ids: Option<Vec<String>>,
    #[serde(rename = "historicProcessInstanceQuery", skip_serializing_if = "Option::is_none")]
    pub historic_process_instance_query: Option<crate::models::HistoricProcessInstanceQueryDto>,
    /// Skip execution listener invocation for activities that are started as part of this request.
    #[serde(rename = "skipCustomListeners", skip_serializing_if = "Option::is_none")]
    pub skip_custom_listeners: Option<bool>,
    /// Skip execution of [input/output variable mappings](https://docs.camunda.org/manual/7.13/user-guide/process-engine/variables/#input-output-variable-mapping) for activities that are started as part of this request.
    #[serde(rename = "skipIoMappings", skip_serializing_if = "Option::is_none")]
    pub skip_io_mappings: Option<bool>,
    /// Set the initial set of variables during restart. By default, the last set of variables is used.
    #[serde(rename = "initialVariables", skip_serializing_if = "Option::is_none")]
    pub initial_variables: Option<bool>,
    /// Do not take over the business key of the historic process instance.
    #[serde(rename = "withoutBusinessKey", skip_serializing_if = "Option::is_none")]
    pub without_business_key: Option<bool>,
    /// **Optional**. A JSON array of instructions that specify which activities to start the process instance at. If this property is omitted, the process instance starts at its default blank start event.
    #[serde(rename = "instructions", skip_serializing_if = "Option::is_none")]
    pub instructions: Option<Vec<crate::models::RestartProcessInstanceModificationInstructionDto>>,
}

impl RestartProcessInstanceDto {
    pub fn new() -> RestartProcessInstanceDto {
        RestartProcessInstanceDto {
            process_instance_ids: None,
            historic_process_instance_query: None,
            skip_custom_listeners: None,
            skip_io_mappings: None,
            initial_variables: None,
            without_business_key: None,
            instructions: None,
        }
    }
}


