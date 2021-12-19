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
pub struct RevenueEstimates {
    /// List of estimates
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::RevenueEstimatesInfo>>,
    /// Frequency: annual or quarterly.
    #[serde(rename = "freq", skip_serializing_if = "Option::is_none")]
    pub freq: Option<String>,
    /// Company symbol.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

impl RevenueEstimates {
    pub fn new() -> RevenueEstimates {
        RevenueEstimates {
            data: None,
            freq: None,
            symbol: None,
        }
    }
}


