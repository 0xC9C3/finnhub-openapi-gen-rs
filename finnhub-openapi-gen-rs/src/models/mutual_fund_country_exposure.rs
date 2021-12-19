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
pub struct MutualFundCountryExposure {
    /// Symbol.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Array of countries and and exposure levels.
    #[serde(rename = "countryExposure", skip_serializing_if = "Option::is_none")]
    pub country_exposure: Option<Vec<crate::models::MutualFundCountryExposureData>>,
}

impl MutualFundCountryExposure {
    pub fn new() -> MutualFundCountryExposure {
        MutualFundCountryExposure {
            symbol: None,
            country_exposure: None,
        }
    }
}


