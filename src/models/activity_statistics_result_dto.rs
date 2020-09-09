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
pub struct ActivityStatisticsResultDto {
    /// The id of the activity the results are aggregated for.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The total number of running process instances of this activity.
    #[serde(rename = "instances", skip_serializing_if = "Option::is_none")]
    pub instances: Option<i32>,
    /// The total number of failed jobs for the running instances. **Note**: Will be `0` (not `null`), if failed jobs were excluded.
    #[serde(rename = "failedJobs", skip_serializing_if = "Option::is_none")]
    pub failed_jobs: Option<i32>,
    /// Each item in the resulting array is an object which contains `incidentType` and `incidentCount`. **Note**: Will be an empty array, if `incidents` or `incidentsForType` were excluded. Furthermore, the array will be also empty if no incidents were found.
    #[serde(rename = "incidents", skip_serializing_if = "Option::is_none")]
    pub incidents: Option<Vec<crate::models::IncidentStatisticsResultDto>>,
}

impl ActivityStatisticsResultDto {
    pub fn new() -> ActivityStatisticsResultDto {
        ActivityStatisticsResultDto {
            id: None,
            instances: None,
            failed_jobs: None,
            incidents: None,
        }
    }
}


