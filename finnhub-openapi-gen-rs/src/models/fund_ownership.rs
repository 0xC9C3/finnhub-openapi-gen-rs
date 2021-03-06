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
pub struct FundOwnership {
    /// Symbol of the company.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Array of investors with detailed information about their holdings.
    #[serde(rename = "ownership", skip_serializing_if = "Option::is_none")]
    pub ownership: Option<Vec<crate::models::FundOwnershipInfo>>,
}

impl FundOwnership {
    pub fn new() -> FundOwnership {
        FundOwnership {
            symbol: None,
            ownership: None,
        }
    }
}


