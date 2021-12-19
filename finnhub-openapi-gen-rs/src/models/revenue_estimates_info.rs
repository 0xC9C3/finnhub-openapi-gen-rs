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
pub struct RevenueEstimatesInfo {
    /// Average revenue estimates including Finnhub's proprietary estimates.
    #[serde(rename = "revenueAvg", skip_serializing_if = "Option::is_none")]
    pub revenue_avg: Option<f32>,
    /// Highest estimate.
    #[serde(rename = "revenueHigh", skip_serializing_if = "Option::is_none")]
    pub revenue_high: Option<f32>,
    /// Lowest estimate.
    #[serde(rename = "revenueLow", skip_serializing_if = "Option::is_none")]
    pub revenue_low: Option<f32>,
    /// Number of Analysts.
    #[serde(rename = "numberAnalysts", skip_serializing_if = "Option::is_none")]
    pub number_analysts: Option<i64>,
    /// Period.
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

impl RevenueEstimatesInfo {
    pub fn new() -> RevenueEstimatesInfo {
        RevenueEstimatesInfo {
            revenue_avg: None,
            revenue_high: None,
            revenue_low: None,
            number_analysts: None,
            period: None,
        }
    }
}

