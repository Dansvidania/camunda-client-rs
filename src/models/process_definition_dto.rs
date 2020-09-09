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
pub struct ProcessDefinitionDto {
    /// The id of the process definition
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The key of the process definition, i.e., the id of the BPMN 2.0 XML process definition.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The category of the process definition.
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The description of the process definition.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the process definition.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The version of the process definition that the engine assigned to it.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    /// The file name of the process definition.
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// The deployment id of the process definition.
    #[serde(rename = "deploymentId", skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// The file name of the process definition diagram, if it exists.
    #[serde(rename = "diagram", skip_serializing_if = "Option::is_none")]
    pub diagram: Option<String>,
    /// A flag indicating whether the definition is suspended or not.
    #[serde(rename = "suspended", skip_serializing_if = "Option::is_none")]
    pub suspended: Option<bool>,
    /// The tenant id of the process definition.
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    /// The version tag of the process definition.
    #[serde(rename = "versionTag", skip_serializing_if = "Option::is_none")]
    pub version_tag: Option<String>,
    /// History time to live value of the process definition. Is used within [History cleanup](https://docs.camunda.org/manual/7.13/user-guide/process-engine/history/#history-cleanup).
    #[serde(rename = "historyTimeToLive", skip_serializing_if = "Option::is_none")]
    pub history_time_to_live: Option<i32>,
    /// A flag indicating whether the process definition is startable in Tasklist or not.
    #[serde(rename = "startableInTasklist", skip_serializing_if = "Option::is_none")]
    pub startable_in_tasklist: Option<bool>,
}

impl ProcessDefinitionDto {
    pub fn new() -> ProcessDefinitionDto {
        ProcessDefinitionDto {
            id: None,
            key: None,
            category: None,
            description: None,
            name: None,
            version: None,
            resource: None,
            deployment_id: None,
            diagram: None,
            suspended: None,
            tenant_id: None,
            version_tag: None,
            history_time_to_live: None,
            startable_in_tasklist: None,
        }
    }
}


