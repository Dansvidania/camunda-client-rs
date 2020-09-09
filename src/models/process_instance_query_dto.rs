/*
 * Camunda BPM REST API
 *
 * OpenApi Spec for Camunda BPM REST API.
 *
 * The version of the OpenAPI document: 7.13.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProcessInstanceQueryDto : A process instance query which defines a group of process instances



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessInstanceQueryDto {
    /// Filter by the deployment the id belongs to.
    #[serde(rename = "deploymentId", skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// Filter by the process definition the instances run on.
    #[serde(rename = "processDefinitionId", skip_serializing_if = "Option::is_none")]
    pub process_definition_id: Option<String>,
    /// Filter by the key of the process definition the instances run on.
    #[serde(rename = "processDefinitionKey", skip_serializing_if = "Option::is_none")]
    pub process_definition_key: Option<String>,
    /// Filter by a list of process definition keys. A process instance must have one of the given process definition keys. Must be a JSON array of Strings.
    #[serde(rename = "processDefinitionKeyIn", skip_serializing_if = "Option::is_none")]
    pub process_definition_key_in: Option<Vec<String>>,
    /// Exclude instances by a list of process definition keys. A process instance must not have one of the given process definition keys. Must be a JSON array of Strings.
    #[serde(rename = "processDefinitionKeyNotIn", skip_serializing_if = "Option::is_none")]
    pub process_definition_key_not_in: Option<Vec<String>>,
    /// Filter by process instance business key.
    #[serde(rename = "businessKey", skip_serializing_if = "Option::is_none")]
    pub business_key: Option<String>,
    /// Filter by process instance business key that the parameter is a substring of.
    #[serde(rename = "businessKeyLike", skip_serializing_if = "Option::is_none")]
    pub business_key_like: Option<String>,
    /// Filter by case instance id.
    #[serde(rename = "caseInstanceId", skip_serializing_if = "Option::is_none")]
    pub case_instance_id: Option<String>,
    /// Restrict query to all process instances that are sub process instances of the given process instance. Takes a process instance id.
    #[serde(rename = "superProcessInstance", skip_serializing_if = "Option::is_none")]
    pub super_process_instance: Option<String>,
    /// Restrict query to all process instances that have the given process instance as a sub process instance. Takes a process instance id.
    #[serde(rename = "subProcessInstance", skip_serializing_if = "Option::is_none")]
    pub sub_process_instance: Option<String>,
    /// Restrict query to all process instances that are sub process instances of the given case instance. Takes a case instance id.
    #[serde(rename = "superCaseInstance", skip_serializing_if = "Option::is_none")]
    pub super_case_instance: Option<String>,
    /// Restrict query to all process instances that have the given case instance as a sub case instance. Takes a case instance id.
    #[serde(rename = "subCaseInstance", skip_serializing_if = "Option::is_none")]
    pub sub_case_instance: Option<String>,
    /// Only include active process instances. Value may only be true, as false is the default behavior.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Only include suspended process instances. Value may only be true, as false is the default behavior.
    #[serde(rename = "suspended", skip_serializing_if = "Option::is_none")]
    pub suspended: Option<bool>,
    /// Filter by a list of process instance ids. Must be a JSON array of Strings.
    #[serde(rename = "processInstanceIds", skip_serializing_if = "Option::is_none")]
    pub process_instance_ids: Option<Vec<String>>,
    /// Filter by presence of incidents. Selects only process instances that have an incident.
    #[serde(rename = "withIncident", skip_serializing_if = "Option::is_none")]
    pub with_incident: Option<bool>,
    /// Filter by the incident id.
    #[serde(rename = "incidentId", skip_serializing_if = "Option::is_none")]
    pub incident_id: Option<String>,
    /// Filter by the incident type. See the User Guide for a list of incident types.
    #[serde(rename = "incidentType", skip_serializing_if = "Option::is_none")]
    pub incident_type: Option<String>,
    /// Filter by the incident message. Exact match.
    #[serde(rename = "incidentMessage", skip_serializing_if = "Option::is_none")]
    pub incident_message: Option<String>,
    /// Filter by the incident message that the parameter is a substring of.
    #[serde(rename = "incidentMessageLike", skip_serializing_if = "Option::is_none")]
    pub incident_message_like: Option<String>,
    /// Filter by a list of tenant ids. A process instance must have one of the given tenant ids. Must be a JSON array of Strings.
    #[serde(rename = "tenantIdIn", skip_serializing_if = "Option::is_none")]
    pub tenant_id_in: Option<Vec<String>>,
    /// Only include process instances which belong to no tenant. Value may only be true, as false is the default behavior.
    #[serde(rename = "withoutTenantId", skip_serializing_if = "Option::is_none")]
    pub without_tenant_id: Option<bool>,
    /// Only include process instances which process definition has no tenant id.
    #[serde(rename = "processDefinitionWithoutTenantId", skip_serializing_if = "Option::is_none")]
    pub process_definition_without_tenant_id: Option<bool>,
    /// Filter by a list of activity ids. A process instance must currently wait in a leaf activity with one of the given activity ids.
    #[serde(rename = "activityIdIn", skip_serializing_if = "Option::is_none")]
    pub activity_id_in: Option<Vec<String>>,
    /// Restrict the query to all process instances that are top level process instances.
    #[serde(rename = "rootProcessInstances", skip_serializing_if = "Option::is_none")]
    pub root_process_instances: Option<bool>,
    /// Restrict the query to all process instances that are leaf instances. (i.e. don't have any sub instances)
    #[serde(rename = "leafProcessInstances", skip_serializing_if = "Option::is_none")]
    pub leaf_process_instances: Option<bool>,
    /// A JSON array to only include process instances that have variables with certain values. The array consists of objects with the three properties `name`, `operator` and `value`. `name` (String) is the variable name, `operator` (String) is the comparison operator to be used and `value` the variable value. The `value` may be String, Number or Boolean.  Valid operator values are: `eq` - equal to; `neq` - not equal to; `gt` - greater than; `gteq` - greater than or equal to; `lt` - lower than; `lteq` - lower than or equal to; `like`.
    #[serde(rename = "variables", skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<crate::models::VariableQueryParameterDto>>,
    /// Match all variable names in this query case-insensitively. If set to true variableName and variablename are treated as equal.
    #[serde(rename = "variableNamesIgnoreCase", skip_serializing_if = "Option::is_none")]
    pub variable_names_ignore_case: Option<bool>,
    /// Match all variable values in this query case-insensitively. If set to true variableValue and variablevalue are treated as equal.
    #[serde(rename = "variableValuesIgnoreCase", skip_serializing_if = "Option::is_none")]
    pub variable_values_ignore_case: Option<bool>,
    /// A JSON array of nested process instance queries with OR semantics. A process instance matches a nested query if it fulfills at least one of the query's predicates. With multiple nested queries, a process instance must fulfill at least one predicate of each query (Conjunctive Normal Form). All process instance query properties can be used except for: `sorting` See the [User guide](https://docs.camunda.org/manual/7.13/user-guide/process-engine/process-engine-api/#or-queries) for more information about OR queries.
    #[serde(rename = "orQueries", skip_serializing_if = "Option::is_none")]
    pub or_queries: Option<Vec<crate::models::ProcessInstanceQueryDto>>,
    /// Apply sorting of the result
    #[serde(rename = "sorting", skip_serializing_if = "Option::is_none")]
    pub sorting: Option<Vec<crate::models::ProcessInstanceQueryDtoSorting>>,
}

impl ProcessInstanceQueryDto {
    /// A process instance query which defines a group of process instances
    pub fn new() -> ProcessInstanceQueryDto {
        ProcessInstanceQueryDto {
            deployment_id: None,
            process_definition_id: None,
            process_definition_key: None,
            process_definition_key_in: None,
            process_definition_key_not_in: None,
            business_key: None,
            business_key_like: None,
            case_instance_id: None,
            super_process_instance: None,
            sub_process_instance: None,
            super_case_instance: None,
            sub_case_instance: None,
            active: None,
            suspended: None,
            process_instance_ids: None,
            with_incident: None,
            incident_id: None,
            incident_type: None,
            incident_message: None,
            incident_message_like: None,
            tenant_id_in: None,
            without_tenant_id: None,
            process_definition_without_tenant_id: None,
            activity_id_in: None,
            root_process_instances: None,
            leaf_process_instances: None,
            variables: None,
            variable_names_ignore_case: None,
            variable_values_ignore_case: None,
            or_queries: None,
            sorting: None,
        }
    }
}


