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
pub struct EarningRelease {
    /// Symbol.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Date.
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Indicates whether the earnings is announced before market open(<code>bmo</code>), after market close(<code>amc</code>), or during market hour(<code>dmh</code>).
    #[serde(rename = "hour", skip_serializing_if = "Option::is_none")]
    pub hour: Option<String>,
    /// Earnings year.
    #[serde(rename = "year", skip_serializing_if = "Option::is_none")]
    pub year: Option<i64>,
    /// Earnings quarter.
    #[serde(rename = "quarter", skip_serializing_if = "Option::is_none")]
    pub quarter: Option<i64>,
    /// EPS estimate.
    #[serde(rename = "epsEstimate", skip_serializing_if = "Option::is_none")]
    pub eps_estimate: Option<f32>,
    /// EPS actual.
    #[serde(rename = "epsActual", skip_serializing_if = "Option::is_none")]
    pub eps_actual: Option<f32>,
    /// Revenue estimate including Finnhub's proprietary estimates.
    #[serde(rename = "revenueEstimate", skip_serializing_if = "Option::is_none")]
    pub revenue_estimate: Option<f32>,
    /// Revenue actual.
    #[serde(rename = "revenueActual", skip_serializing_if = "Option::is_none")]
    pub revenue_actual: Option<f32>,
}

impl EarningRelease {
    pub fn new() -> EarningRelease {
        EarningRelease {
            symbol: None,
            date: None,
            hour: None,
            year: None,
            quarter: None,
            eps_estimate: None,
            eps_actual: None,
            revenue_estimate: None,
            revenue_actual: None,
        }
    }
}

