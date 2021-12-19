/*
 * Finnhub API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FdaComitteeMeeting {
    /// Start time of the event in EST.
    #[serde(rename = "fromDate", skip_serializing_if = "Option::is_none")]
    pub from_date: Option<String>,
    /// End time of the event in EST.
    #[serde(rename = "toDate", skip_serializing_if = "Option::is_none")]
    pub to_date: Option<String>,
    /// Event's description.
    #[serde(rename = "eventDescription", skip_serializing_if = "Option::is_none")]
    pub event_description: Option<String>,
    /// URL.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl FdaComitteeMeeting {
    pub fn new() -> FdaComitteeMeeting {
        FdaComitteeMeeting {
            from_date: None,
            to_date: None,
            event_description: None,
            url: None,
        }
    }
}

