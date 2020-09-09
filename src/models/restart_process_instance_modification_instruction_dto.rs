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
pub struct RestartProcessInstanceModificationInstructionDto {
    /// **Mandatory**. One of the following values: `startBeforeActivity`, `startAfterActivity`, `startTransition`.  * A `startBeforeActivity` instruction requests to enter a given activity. * A `startAfterActivity` instruction requests to execute the single outgoing sequence flow of a given activity. * A `startTransition` instruction requests to execute a specific sequence flow.
    #[serde(rename = "type")]
    pub _type: Type,
    /// **Can be used with instructions of types** `startBeforeActivity` and `startAfterActivity`. Specifies the sequence flow to start.
    #[serde(rename = "activityId", skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    /// **Can be used with instructions of types** `startTransition`. Specifies the sequence flow to start.
    #[serde(rename = "transitionId", skip_serializing_if = "Option::is_none")]
    pub transition_id: Option<String>,
}

impl RestartProcessInstanceModificationInstructionDto {
    pub fn new(_type: Type) -> RestartProcessInstanceModificationInstructionDto {
        RestartProcessInstanceModificationInstructionDto {
            _type,
            activity_id: None,
            transition_id: None,
        }
    }
}

/// **Mandatory**. One of the following values: `startBeforeActivity`, `startAfterActivity`, `startTransition`.  * A `startBeforeActivity` instruction requests to enter a given activity. * A `startAfterActivity` instruction requests to execute the single outgoing sequence flow of a given activity. * A `startTransition` instruction requests to execute a specific sequence flow.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "startBeforeActivity")]
    StartBeforeActivity,
    #[serde(rename = "startAfterActivity")]
    StartAfterActivity,
    #[serde(rename = "startTransition")]
    StartTransition,
}
