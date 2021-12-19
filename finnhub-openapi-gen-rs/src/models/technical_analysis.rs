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
pub struct TechnicalAnalysis {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<Box<crate::models::Indicator>>,
    /// Aggregate Signal
    #[serde(rename = "signal", skip_serializing_if = "Option::is_none")]
    pub signal: Option<String>,
}

impl TechnicalAnalysis {
    pub fn new() -> TechnicalAnalysis {
        TechnicalAnalysis {
            count: None,
            signal: None,
        }
    }
}

