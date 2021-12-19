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
pub struct EarningsEstimatesInfo {
    /// Average EPS estimates including Finnhub's proprietary estimates.
    #[serde(rename = "epsAvg", skip_serializing_if = "Option::is_none")]
    pub eps_avg: Option<f32>,
    /// Highest estimate.
    #[serde(rename = "epsHigh", skip_serializing_if = "Option::is_none")]
    pub eps_high: Option<f32>,
    /// Lowest estimate.
    #[serde(rename = "epsLow", skip_serializing_if = "Option::is_none")]
    pub eps_low: Option<f32>,
    /// Number of Analysts.
    #[serde(rename = "numberAnalysts", skip_serializing_if = "Option::is_none")]
    pub number_analysts: Option<i64>,
    /// Period.
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

impl EarningsEstimatesInfo {
    pub fn new() -> EarningsEstimatesInfo {
        EarningsEstimatesInfo {
            eps_avg: None,
            eps_high: None,
            eps_low: None,
            number_analysts: None,
            period: None,
        }
    }
}

