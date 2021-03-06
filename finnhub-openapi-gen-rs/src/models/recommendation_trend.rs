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
pub struct RecommendationTrend {
    /// Company symbol.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Number of recommendations that fall into the Buy category
    #[serde(rename = "buy", skip_serializing_if = "Option::is_none")]
    pub buy: Option<i64>,
    /// Number of recommendations that fall into the Hold category
    #[serde(rename = "hold", skip_serializing_if = "Option::is_none")]
    pub hold: Option<i64>,
    /// Updated period
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    /// Number of recommendations that fall into the Sell category
    #[serde(rename = "sell", skip_serializing_if = "Option::is_none")]
    pub sell: Option<i64>,
    /// Number of recommendations that fall into the Strong Buy category
    #[serde(rename = "strongBuy", skip_serializing_if = "Option::is_none")]
    pub strong_buy: Option<i64>,
    /// Number of recommendations that fall into the Strong Sell category
    #[serde(rename = "strongSell", skip_serializing_if = "Option::is_none")]
    pub strong_sell: Option<i64>,
}

impl RecommendationTrend {
    pub fn new() -> RecommendationTrend {
        RecommendationTrend {
            symbol: None,
            buy: None,
            hold: None,
            period: None,
            sell: None,
            strong_buy: None,
            strong_sell: None,
        }
    }
}


