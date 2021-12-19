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
pub struct SymbolLookup {
    /// Array of search results.
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<crate::models::SymbolLookupInfo>>,
    /// Number of results.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

impl SymbolLookup {
    pub fn new() -> SymbolLookup {
        SymbolLookup {
            result: None,
            count: None,
        }
    }
}


