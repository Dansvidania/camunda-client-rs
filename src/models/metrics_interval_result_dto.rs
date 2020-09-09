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
pub struct MetricsIntervalResultDto {
    /// The interval timestamp.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// The name of the metric.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The reporter of the metric. `null` if the metrics are aggregated by reporter.
    #[serde(rename = "reporter", skip_serializing_if = "Option::is_none")]
    pub reporter: Option<String>,
    /// The value of the metric aggregated by the interval.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

impl MetricsIntervalResultDto {
    pub fn new() -> MetricsIntervalResultDto {
        MetricsIntervalResultDto {
            timestamp: None,
            name: None,
            reporter: None,
            value: None,
        }
    }
}


