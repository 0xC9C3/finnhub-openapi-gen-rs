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
pub struct IpoEvent {
    /// Symbol.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// IPO date.
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Exchange.
    #[serde(rename = "exchange", skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    /// Company's name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// IPO status. Can take 1 of the following values: <code>expected</code>,<code>priced</code>,<code>withdrawn</code>,<code>filed</code>
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Projected price or price range.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Number of shares offered during the IPO.
    #[serde(rename = "numberOfShares", skip_serializing_if = "Option::is_none")]
    pub number_of_shares: Option<f32>,
    /// Total shares value.
    #[serde(rename = "totalSharesValue", skip_serializing_if = "Option::is_none")]
    pub total_shares_value: Option<f32>,
}

impl IpoEvent {
    pub fn new() -> IpoEvent {
        IpoEvent {
            symbol: None,
            date: None,
            exchange: None,
            name: None,
            status: None,
            price: None,
            number_of_shares: None,
            total_shares_value: None,
        }
    }
}


