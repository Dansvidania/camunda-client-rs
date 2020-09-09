/*
 * Camunda BPM REST API
 *
 * OpenApi Spec for Camunda BPM REST API.
 *
 * The version of the OpenAPI document: 7.13.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use reqwest;

use super::{Error, configuration};

pub struct ProcessInstanceApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl ProcessInstanceApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> ProcessInstanceApiClient {
        ProcessInstanceApiClient {
            configuration,
        }
    }
}

pub trait ProcessInstanceApi {
    fn delete_async_historic_query_based(&self, delete_process_instances_dto: Option<crate::models::DeleteProcessInstancesDto>) -> Result<crate::models::BatchDto, Error>;
    fn delete_process_instance(&self, id: &str, skip_custom_listeners: Option<bool>, skip_io_mappings: Option<bool>, skip_subprocesses: Option<bool>, fail_if_not_exists: Option<bool>) -> Result<(), Error>;
    fn delete_process_instance_variable(&self, id: &str, var_name: &str) -> Result<(), Error>;
    fn delete_process_instances_async_operation(&self, delete_process_instances_dto: Option<crate::models::DeleteProcessInstancesDto>) -> Result<crate::models::BatchDto, Error>;
    fn get_activity_instance_tree(&self, id: &str) -> Result<crate::models::ActivityInstanceDto, Error>;
    fn get_process_instance_variable(&self, id: &str, var_name: &str, deserialize_value: Option<bool>) -> Result<crate::models::VariableValueDto, Error>;
    fn get_process_instance_variable_binary(&self, id: &str, var_name: &str) -> Result<std::path::PathBuf, Error>;
    fn get_process_instance_variables(&self, id: &str, deserialize_value: Option<bool>) -> Result<::std::collections::HashMap<String, crate::models::VariableValueDto>, Error>;
    fn get_process_instances(&self, sort_by: Option<&str>, sort_order: Option<&str>, first_result: Option<i32>, max_results: Option<i32>, process_instance_ids: Option<&str>, business_key: Option<&str>, business_key_like: Option<&str>, case_instance_id: Option<&str>, process_definition_id: Option<&str>, process_definition_key: Option<&str>, process_definition_key_in: Option<&str>, process_definition_key_not_in: Option<&str>, deployment_id: Option<&str>, super_process_instance: Option<&str>, sub_process_instance: Option<&str>, super_case_instance: Option<&str>, sub_case_instance: Option<&str>, active: Option<bool>, suspended: Option<bool>, with_incident: Option<bool>, incident_id: Option<&str>, incident_type: Option<&str>, incident_message: Option<&str>, incident_message_like: Option<&str>, tenant_id_in: Option<&str>, without_tenant_id: Option<bool>, process_definition_without_tenant_id: Option<bool>, activity_id_in: Option<&str>, root_process_instances: Option<bool>, leaf_process_instances: Option<bool>, variables: Option<&str>, variable_names_ignore_case: Option<bool>, variable_values_ignore_case: Option<bool>) -> Result<Vec<crate::models::ProcessInstanceDto>, Error>;
    fn get_process_instances_count(&self, process_instance_ids: Option<&str>, business_key: Option<&str>, business_key_like: Option<&str>, case_instance_id: Option<&str>, process_definition_id: Option<&str>, process_definition_key: Option<&str>, process_definition_key_in: Option<&str>, process_definition_key_not_in: Option<&str>, deployment_id: Option<&str>, super_process_instance: Option<&str>, sub_process_instance: Option<&str>, super_case_instance: Option<&str>, sub_case_instance: Option<&str>, active: Option<bool>, suspended: Option<bool>, with_incident: Option<bool>, incident_id: Option<&str>, incident_type: Option<&str>, incident_message: Option<&str>, incident_message_like: Option<&str>, tenant_id_in: Option<&str>, without_tenant_id: Option<bool>, process_definition_without_tenant_id: Option<bool>, activity_id_in: Option<&str>, root_process_instances: Option<bool>, leaf_process_instances: Option<bool>, variables: Option<&str>, variable_names_ignore_case: Option<bool>, variable_values_ignore_case: Option<bool>) -> Result<crate::models::CountResultDto, Error>;
    fn modify_process_instance(&self, id: &str, process_instance_modification_dto: Option<crate::models::ProcessInstanceModificationDto>) -> Result<(), Error>;
    fn modify_process_instance_async_operation(&self, id: &str, process_instance_modification_dto: Option<crate::models::ProcessInstanceModificationDto>) -> Result<crate::models::BatchDto, Error>;
    fn modify_process_instance_variables(&self, id: &str, patch_variables_dto: Option<crate::models::PatchVariablesDto>) -> Result<(), Error>;
    fn query_process_instances(&self, first_result: Option<i32>, max_results: Option<i32>, process_instance_query_dto: Option<crate::models::ProcessInstanceQueryDto>) -> Result<Vec<crate::models::ProcessInstanceDto>, Error>;
    fn query_process_instances_count(&self, process_instance_query_dto: Option<crate::models::ProcessInstanceQueryDto>) -> Result<crate::models::CountResultDto, Error>;
    fn set_process_instance_variable(&self, id: &str, var_name: &str, variable_value_dto: Option<crate::models::VariableValueDto>) -> Result<(), Error>;
    fn set_process_instance_variable_binary(&self, id: &str, var_name: &str, data: Option<std::path::PathBuf>, value_type: Option<&str>) -> Result<(), Error>;
    fn set_retries_by_process(&self, set_job_retries_by_process_dto: Option<crate::models::SetJobRetriesByProcessDto>) -> Result<crate::models::BatchDto, Error>;
    fn set_retries_by_process_historic_query_based(&self, set_job_retries_by_process_dto: Option<crate::models::SetJobRetriesByProcessDto>) -> Result<crate::models::BatchDto, Error>;
    fn update_suspension_state(&self, process_instance_suspension_state_dto: Option<crate::models::ProcessInstanceSuspensionStateDto>) -> Result<(), Error>;
    fn update_suspension_state_async_operation(&self, process_instance_suspension_state_async_dto: Option<crate::models::ProcessInstanceSuspensionStateAsyncDto>) -> Result<crate::models::BatchDto, Error>;
    fn update_suspension_state_by_id(&self, id: &str, suspension_state_dto: Option<crate::models::SuspensionStateDto>) -> Result<(), Error>;
}

impl ProcessInstanceApi for ProcessInstanceApiClient {
    fn delete_async_historic_query_based(&self, delete_process_instances_dto: Option<crate::models::DeleteProcessInstancesDto>) -> Result<crate::models::BatchDto, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/process-instance/delete-historic-query-based", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&delete_process_instances_dto);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn delete_process_instance(&self, id: &str, skip_custom_listeners: Option<bool>, skip_io_mappings: Option<bool>, skip_subprocesses: Option<bool>, fail_if_not_exists: Option<bool>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/process-instance/{id}", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref s) = skip_custom_listeners {
            req_builder = req_builder.query(&[("skipCustomListeners", &s.to_string())]);
        }
        if let Some(ref s) = skip_io_mappings {
            req_builder = req_builder.query(&[("skipIoMappings", &s.to_string())]);
        }
        if let Some(ref s) = skip_subprocesses {
            req_builder = req_builder.query(&[("skipSubprocesses", &s.to_string())]);
        }
        if let Some(ref s) = fail_if_not_exists {
            req_builder = req_builder.query(&[("failIfNotExists", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn delete_process_instance_variable(&self, id: &str, var_name: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/process-instance/{id}/variables/{varName}", configuration.base_path, id=crate::apis::urlencode(id), varName=crate::apis::urlencode(var_name));
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn delete_process_instances_async_operation(&self, delete_process_instances_dto: Option<crate::models::DeleteProcessInstancesDto>) -> Result<crate::models::BatchDto, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/process-instance/delete", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&delete_process_instances_dto);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_activity_instance_tree(&self, id: &str) -> Result<crate::models::ActivityInstanceDto, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/process-instance/{id}/activity-instances", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_process_instance_variable(&self, id: &str, var_name: &str, deserialize_value: Option<bool>) -> Result<crate::models::VariableValueDto, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/process-instance/{id}/variables/{varName}", configuration.base_path, id=crate::apis::urlencode(id), varName=crate::apis::urlencode(var_name));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = deserialize_value {
            req_builder = req_builder.query(&[("deserializeValue", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_process_instance_variable_binary(&self, id: &str, var_name: &str) -> Result<std::path::PathBuf, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/process-instance/{id}/variables/{varName}/data", configuration.base_path, id=crate::apis::urlencode(id), varName=crate::apis::urlencode(var_name));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_process_instance_variables(&self, id: &str, deserialize_value: Option<bool>) -> Result<::std::collections::HashMap<String, crate::models::VariableValueDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/process-instance/{id}/variables", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = deserialize_value {
            req_builder = req_builder.query(&[("deserializeValue", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_process_instances(&self, sort_by: Option<&str>, sort_order: Option<&str>, first_result: Option<i32>, max_results: Option<i32>, process_instance_ids: Option<&str>, business_key: Option<&str>, business_key_like: Option<&str>, case_instance_id: Option<&str>, process_definition_id: Option<&str>, process_definition_key: Option<&str>, process_definition_key_in: Option<&str>, process_definition_key_not_in: Option<&str>, deployment_id: Option<&str>, super_process_instance: Option<&str>, sub_process_instance: Option<&str>, super_case_instance: Option<&str>, sub_case_instance: Option<&str>, active: Option<bool>, suspended: Option<bool>, with_incident: Option<bool>, incident_id: Option<&str>, incident_type: Option<&str>, incident_message: Option<&str>, incident_message_like: Option<&str>, tenant_id_in: Option<&str>, without_tenant_id: Option<bool>, process_definition_without_tenant_id: Option<bool>, activity_id_in: Option<&str>, root_process_instances: Option<bool>, leaf_process_instances: Option<bool>, variables: Option<&str>, variable_names_ignore_case: Option<bool>, variable_values_ignore_case: Option<bool>) -> Result<Vec<crate::models::ProcessInstanceDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/process-instance", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = sort_by {
            req_builder = req_builder.query(&[("sortBy", &s.to_string())]);
        }
        if let Some(ref s) = sort_order {
            req_builder = req_builder.query(&[("sortOrder", &s.to_string())]);
        }
        if let Some(ref s) = first_result {
            req_builder = req_builder.query(&[("firstResult", &s.to_string())]);
        }
        if let Some(ref s) = max_results {
            req_builder = req_builder.query(&[("maxResults", &s.to_string())]);
        }
        if let Some(ref s) = process_instance_ids {
            req_builder = req_builder.query(&[("processInstanceIds", &s.to_string())]);
        }
        if let Some(ref s) = business_key {
            req_builder = req_builder.query(&[("businessKey", &s.to_string())]);
        }
        if let Some(ref s) = business_key_like {
            req_builder = req_builder.query(&[("businessKeyLike", &s.to_string())]);
        }
        if let Some(ref s) = case_instance_id {
            req_builder = req_builder.query(&[("caseInstanceId", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_id {
            req_builder = req_builder.query(&[("processDefinitionId", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_key {
            req_builder = req_builder.query(&[("processDefinitionKey", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_key_in {
            req_builder = req_builder.query(&[("processDefinitionKeyIn", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_key_not_in {
            req_builder = req_builder.query(&[("processDefinitionKeyNotIn", &s.to_string())]);
        }
        if let Some(ref s) = deployment_id {
            req_builder = req_builder.query(&[("deploymentId", &s.to_string())]);
        }
        if let Some(ref s) = super_process_instance {
            req_builder = req_builder.query(&[("superProcessInstance", &s.to_string())]);
        }
        if let Some(ref s) = sub_process_instance {
            req_builder = req_builder.query(&[("subProcessInstance", &s.to_string())]);
        }
        if let Some(ref s) = super_case_instance {
            req_builder = req_builder.query(&[("superCaseInstance", &s.to_string())]);
        }
        if let Some(ref s) = sub_case_instance {
            req_builder = req_builder.query(&[("subCaseInstance", &s.to_string())]);
        }
        if let Some(ref s) = active {
            req_builder = req_builder.query(&[("active", &s.to_string())]);
        }
        if let Some(ref s) = suspended {
            req_builder = req_builder.query(&[("suspended", &s.to_string())]);
        }
        if let Some(ref s) = with_incident {
            req_builder = req_builder.query(&[("withIncident", &s.to_string())]);
        }
        if let Some(ref s) = incident_id {
            req_builder = req_builder.query(&[("incidentId", &s.to_string())]);
        }
        if let Some(ref s) = incident_type {
            req_builder = req_builder.query(&[("incidentType", &s.to_string())]);
        }
        if let Some(ref s) = incident_message {
            req_builder = req_builder.query(&[("incidentMessage", &s.to_string())]);
        }
        if let Some(ref s) = incident_message_like {
            req_builder = req_builder.query(&[("incidentMessageLike", &s.to_string())]);
        }
        if let Some(ref s) = tenant_id_in {
            req_builder = req_builder.query(&[("tenantIdIn", &s.to_string())]);
        }
        if let Some(ref s) = without_tenant_id {
            req_builder = req_builder.query(&[("withoutTenantId", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_without_tenant_id {
            req_builder = req_builder.query(&[("processDefinitionWithoutTenantId", &s.to_string())]);
        }
        if let Some(ref s) = activity_id_in {
            req_builder = req_builder.query(&[("activityIdIn", &s.to_string())]);
        }
        if let Some(ref s) = root_process_instances {
            req_builder = req_builder.query(&[("rootProcessInstances", &s.to_string())]);
        }
        if let Some(ref s) = leaf_process_instances {
            req_builder = req_builder.query(&[("leafProcessInstances", &s.to_string())]);
        }
        if let Some(ref s) = variables {
            req_builder = req_builder.query(&[("variables", &s.to_string())]);
        }
        if let Some(ref s) = variable_names_ignore_case {
            req_builder = req_builder.query(&[("variableNamesIgnoreCase", &s.to_string())]);
        }
        if let Some(ref s) = variable_values_ignore_case {
            req_builder = req_builder.query(&[("variableValuesIgnoreCase", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_process_instances_count(&self, process_instance_ids: Option<&str>, business_key: Option<&str>, business_key_like: Option<&str>, case_instance_id: Option<&str>, process_definition_id: Option<&str>, process_definition_key: Option<&str>, process_definition_key_in: Option<&str>, process_definition_key_not_in: Option<&str>, deployment_id: Option<&str>, super_process_instance: Option<&str>, sub_process_instance: Option<&str>, super_case_instance: Option<&str>, sub_case_instance: Option<&str>, active: Option<bool>, suspended: Option<bool>, with_incident: Option<bool>, incident_id: Option<&str>, incident_type: Option<&str>, incident_message: Option<&str>, incident_message_like: Option<&str>, tenant_id_in: Option<&str>, without_tenant_id: Option<bool>, process_definition_without_tenant_id: Option<bool>, activity_id_in: Option<&str>, root_process_instances: Option<bool>, leaf_process_instances: Option<bool>, variables: Option<&str>, variable_names_ignore_case: Option<bool>, variable_values_ignore_case: Option<bool>) -> Result<crate::models::CountResultDto, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/process-instance/count", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = process_instance_ids {
            req_builder = req_builder.query(&[("processInstanceIds", &s.to_string())]);
        }
        if let Some(ref s) = business_key {
            req_builder = req_builder.query(&[("businessKey", &s.to_string())]);
        }
        if let Some(ref s) = business_key_like {
            req_builder = req_builder.query(&[("businessKeyLike", &s.to_string())]);
        }
        if let Some(ref s) = case_instance_id {
            req_builder = req_builder.query(&[("caseInstanceId", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_id {
            req_builder = req_builder.query(&[("processDefinitionId", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_key {
            req_builder = req_builder.query(&[("processDefinitionKey", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_key_in {
            req_builder = req_builder.query(&[("processDefinitionKeyIn", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_key_not_in {
            req_builder = req_builder.query(&[("processDefinitionKeyNotIn", &s.to_string())]);
        }
        if let Some(ref s) = deployment_id {
            req_builder = req_builder.query(&[("deploymentId", &s.to_string())]);
        }
        if let Some(ref s) = super_process_instance {
            req_builder = req_builder.query(&[("superProcessInstance", &s.to_string())]);
        }
        if let Some(ref s) = sub_process_instance {
            req_builder = req_builder.query(&[("subProcessInstance", &s.to_string())]);
        }
        if let Some(ref s) = super_case_instance {
            req_builder = req_builder.query(&[("superCaseInstance", &s.to_string())]);
        }
        if let Some(ref s) = sub_case_instance {
            req_builder = req_builder.query(&[("subCaseInstance", &s.to_string())]);
        }
        if let Some(ref s) = active {
            req_builder = req_builder.query(&[("active", &s.to_string())]);
        }
        if let Some(ref s) = suspended {
            req_builder = req_builder.query(&[("suspended", &s.to_string())]);
        }
        if let Some(ref s) = with_incident {
            req_builder = req_builder.query(&[("withIncident", &s.to_string())]);
        }
        if let Some(ref s) = incident_id {
            req_builder = req_builder.query(&[("incidentId", &s.to_string())]);
        }
        if let Some(ref s) = incident_type {
            req_builder = req_builder.query(&[("incidentType", &s.to_string())]);
        }
        if let Some(ref s) = incident_message {
            req_builder = req_builder.query(&[("incidentMessage", &s.to_string())]);
        }
        if let Some(ref s) = incident_message_like {
            req_builder = req_builder.query(&[("incidentMessageLike", &s.to_string())]);
        }
        if let Some(ref s) = tenant_id_in {
            req_builder = req_builder.query(&[("tenantIdIn", &s.to_string())]);
        }
        if let Some(ref s) = without_tenant_id {
            req_builder = req_builder.query(&[("withoutTenantId", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_without_tenant_id {
            req_builder = req_builder.query(&[("processDefinitionWithoutTenantId", &s.to_string())]);
        }
        if let Some(ref s) = activity_id_in {
            req_builder = req_builder.query(&[("activityIdIn", &s.to_string())]);
        }
        if let Some(ref s) = root_process_instances {
            req_builder = req_builder.query(&[("rootProcessInstances", &s.to_string())]);
        }
        if let Some(ref s) = leaf_process_instances {
            req_builder = req_builder.query(&[("leafProcessInstances", &s.to_string())]);
        }
        if let Some(ref s) = variables {
            req_builder = req_builder.query(&[("variables", &s.to_string())]);
        }
        if let Some(ref s) = variable_names_ignore_case {
            req_builder = req_builder.query(&[("variableNamesIgnoreCase", &s.to_string())]);
        }
        if let Some(ref s) = variable_values_ignore_case {
            req_builder = req_builder.query(&[("variableValuesIgnoreCase", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn modify_process_instance(&self, id: &str, process_instance_modification_dto: Option<crate::models::ProcessInstanceModificationDto>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/process-instance/{id}/modification", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&process_instance_modification_dto);

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn modify_process_instance_async_operation(&self, id: &str, process_instance_modification_dto: Option<crate::models::ProcessInstanceModificationDto>) -> Result<crate::models::BatchDto, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/process-instance/{id}/modification-async", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&process_instance_modification_dto);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn modify_process_instance_variables(&self, id: &str, patch_variables_dto: Option<crate::models::PatchVariablesDto>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/process-instance/{id}/variables", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&patch_variables_dto);

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn query_process_instances(&self, first_result: Option<i32>, max_results: Option<i32>, process_instance_query_dto: Option<crate::models::ProcessInstanceQueryDto>) -> Result<Vec<crate::models::ProcessInstanceDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/process-instance", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref s) = first_result {
            req_builder = req_builder.query(&[("firstResult", &s.to_string())]);
        }
        if let Some(ref s) = max_results {
            req_builder = req_builder.query(&[("maxResults", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&process_instance_query_dto);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn query_process_instances_count(&self, process_instance_query_dto: Option<crate::models::ProcessInstanceQueryDto>) -> Result<crate::models::CountResultDto, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/process-instance/count", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&process_instance_query_dto);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn set_process_instance_variable(&self, id: &str, var_name: &str, variable_value_dto: Option<crate::models::VariableValueDto>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/process-instance/{id}/variables/{varName}", configuration.base_path, id=crate::apis::urlencode(id), varName=crate::apis::urlencode(var_name));
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&variable_value_dto);

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn set_process_instance_variable_binary(&self, id: &str, var_name: &str, data: Option<std::path::PathBuf>, value_type: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/process-instance/{id}/variables/{varName}/data", configuration.base_path, id=crate::apis::urlencode(id), varName=crate::apis::urlencode(var_name));
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        let mut form = reqwest::multipart::Form::new();
        if let Some(param_value) = data {
            form = form.file("data", param_value)?;
        }
        if let Some(param_value) = value_type {
            form = form.text("valueType", param_value.to_string());
        }
        req_builder = req_builder.multipart(form);

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn set_retries_by_process(&self, set_job_retries_by_process_dto: Option<crate::models::SetJobRetriesByProcessDto>) -> Result<crate::models::BatchDto, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/process-instance/job-retries", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&set_job_retries_by_process_dto);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn set_retries_by_process_historic_query_based(&self, set_job_retries_by_process_dto: Option<crate::models::SetJobRetriesByProcessDto>) -> Result<crate::models::BatchDto, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/process-instance/job-retries-historic-query-based", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&set_job_retries_by_process_dto);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn update_suspension_state(&self, process_instance_suspension_state_dto: Option<crate::models::ProcessInstanceSuspensionStateDto>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/process-instance/suspended", configuration.base_path);
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&process_instance_suspension_state_dto);

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn update_suspension_state_async_operation(&self, process_instance_suspension_state_async_dto: Option<crate::models::ProcessInstanceSuspensionStateAsyncDto>) -> Result<crate::models::BatchDto, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/process-instance/suspended-async", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&process_instance_suspension_state_async_dto);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn update_suspension_state_by_id(&self, id: &str, suspension_state_dto: Option<crate::models::SuspensionStateDto>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/process-instance/{id}/suspended", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&suspension_state_dto);

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

}
