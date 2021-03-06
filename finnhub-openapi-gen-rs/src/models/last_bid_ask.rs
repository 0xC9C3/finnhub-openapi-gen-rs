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
pub struct LastBidAsk {
    /// Bid price.
    #[serde(rename = "b", skip_serializing_if = "Option::is_none")]
    pub b: Option<f32>,
    /// Ask price.
    #[serde(rename = "a", skip_serializing_if = "Option::is_none")]
    pub a: Option<f32>,
    /// Bid volume.
    #[serde(rename = "bv", skip_serializing_if = "Option::is_none")]
    pub bv: Option<f32>,
    /// Ask volume.
    #[serde(rename = "av", skip_serializing_if = "Option::is_none")]
    pub av: Option<f32>,
    /// Reference UNIX timestamp in ms.
    #[serde(rename = "t", skip_serializing_if = "Option::is_none")]
    pub t: Option<i64>,
}

impl LastBidAsk {
    pub fn new() -> LastBidAsk {
        LastBidAsk {
            b: None,
            a: None,
            bv: None,
            av: None,
            t: None,
        }
    }
}


