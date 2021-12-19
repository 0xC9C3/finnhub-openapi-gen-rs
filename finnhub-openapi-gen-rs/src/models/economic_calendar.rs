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
pub struct EconomicCalendar {
    /// Array of economic events.
    #[serde(rename = "economicCalendar", skip_serializing_if = "Option::is_none")]
    pub economic_calendar: Option<Vec<crate::models::EconomicEvent>>,
}

impl EconomicCalendar {
    pub fn new() -> EconomicCalendar {
        EconomicCalendar {
            economic_calendar: None,
        }
    }
}


