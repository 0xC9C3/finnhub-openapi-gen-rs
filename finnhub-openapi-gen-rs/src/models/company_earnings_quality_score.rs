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
pub struct CompanyEarningsQualityScore {
    /// Symbol
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Frequency
    #[serde(rename = "freq", skip_serializing_if = "Option::is_none")]
    pub freq: Option<String>,
    /// Array of earnings quality score.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::CompanyEarningsQualityScoreData>>,
}

impl CompanyEarningsQualityScore {
    pub fn new() -> CompanyEarningsQualityScore {
        CompanyEarningsQualityScore {
            symbol: None,
            freq: None,
            data: None,
        }
    }
}


