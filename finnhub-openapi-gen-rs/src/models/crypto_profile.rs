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
pub struct CryptoProfile {
    /// Long name.
    #[serde(rename = "longName", skip_serializing_if = "Option::is_none")]
    pub long_name: Option<String>,
    /// Name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Project's website.
    #[serde(rename = "website", skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    /// Market capitalization.
    #[serde(rename = "marketCap", skip_serializing_if = "Option::is_none")]
    pub market_cap: Option<f32>,
    /// Total supply.
    #[serde(rename = "totalSupply", skip_serializing_if = "Option::is_none")]
    pub total_supply: Option<f32>,
    /// Max supply.
    #[serde(rename = "maxSupply", skip_serializing_if = "Option::is_none")]
    pub max_supply: Option<f32>,
    /// Circulating supply.
    #[serde(rename = "circulatingSupply", skip_serializing_if = "Option::is_none")]
    pub circulating_supply: Option<f32>,
    /// Logo image.
    #[serde(rename = "logo", skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    /// Launch date.
    #[serde(rename = "launchDate", skip_serializing_if = "Option::is_none")]
    pub launch_date: Option<String>,
    /// Proof type.
    #[serde(rename = "proofType", skip_serializing_if = "Option::is_none")]
    pub proof_type: Option<String>,
}

impl CryptoProfile {
    pub fn new() -> CryptoProfile {
        CryptoProfile {
            long_name: None,
            name: None,
            description: None,
            website: None,
            market_cap: None,
            total_supply: None,
            max_supply: None,
            circulating_supply: None,
            logo: None,
            launch_date: None,
            proof_type: None,
        }
    }
}

