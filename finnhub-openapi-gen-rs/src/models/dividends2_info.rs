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
pub struct Dividends2Info {
    /// Ex-Dividend date.
    #[serde(rename = "exDate", skip_serializing_if = "Option::is_none")]
    pub ex_date: Option<String>,
    /// Amount in local currency.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
}

impl Dividends2Info {
    pub fn new() -> Dividends2Info {
        Dividends2Info {
            ex_date: None,
            amount: None,
        }
    }
}

