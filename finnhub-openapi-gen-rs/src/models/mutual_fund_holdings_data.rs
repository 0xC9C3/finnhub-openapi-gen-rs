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
pub struct MutualFundHoldingsData {
    /// Symbol description
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Security name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// ISIN.
    #[serde(rename = "isin", skip_serializing_if = "Option::is_none")]
    pub isin: Option<String>,
    /// CUSIP.
    #[serde(rename = "cusip", skip_serializing_if = "Option::is_none")]
    pub cusip: Option<String>,
    /// Number of shares.
    #[serde(rename = "share", skip_serializing_if = "Option::is_none")]
    pub share: Option<f32>,
    /// Portfolio's percent
    #[serde(rename = "percent", skip_serializing_if = "Option::is_none")]
    pub percent: Option<f32>,
    /// Market value
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f32>,
}

impl MutualFundHoldingsData {
    pub fn new() -> MutualFundHoldingsData {
        MutualFundHoldingsData {
            symbol: None,
            name: None,
            isin: None,
            cusip: None,
            share: None,
            percent: None,
            value: None,
        }
    }
}


