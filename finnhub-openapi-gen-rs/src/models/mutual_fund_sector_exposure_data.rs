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
pub struct MutualFundSectorExposureData {
    /// Sector
    #[serde(rename = "sector", skip_serializing_if = "Option::is_none")]
    pub sector: Option<String>,
    /// Percent of exposure.
    #[serde(rename = "exposure", skip_serializing_if = "Option::is_none")]
    pub exposure: Option<f32>,
}

impl MutualFundSectorExposureData {
    pub fn new() -> MutualFundSectorExposureData {
        MutualFundSectorExposureData {
            sector: None,
            exposure: None,
        }
    }
}


