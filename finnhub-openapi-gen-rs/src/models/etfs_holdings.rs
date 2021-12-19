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
pub struct EtfsHoldings {
    /// ETF symbol.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Holdings update date.
    #[serde(rename = "atDate", skip_serializing_if = "Option::is_none")]
    pub at_date: Option<String>,
    /// Number of holdings.
    #[serde(rename = "numberOfHoldings", skip_serializing_if = "Option::is_none")]
    pub number_of_holdings: Option<i64>,
    /// Array of holdings.
    #[serde(rename = "holdings", skip_serializing_if = "Option::is_none")]
    pub holdings: Option<Vec<crate::models::EtfHoldingsData>>,
}

impl EtfsHoldings {
    pub fn new() -> EtfsHoldings {
        EtfsHoldings {
            symbol: None,
            at_date: None,
            number_of_holdings: None,
            holdings: None,
        }
    }
}

