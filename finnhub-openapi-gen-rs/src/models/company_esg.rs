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
pub struct CompanyEsg {
    /// symbol
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Total ESG Score
    #[serde(rename = "totalESGScore", skip_serializing_if = "Option::is_none")]
    pub total_esg_score: Option<f32>,
    /// Environment Score
    #[serde(rename = "environmentScore", skip_serializing_if = "Option::is_none")]
    pub environment_score: Option<f32>,
    /// Governance Score
    #[serde(rename = "governanceScore", skip_serializing_if = "Option::is_none")]
    pub governance_score: Option<f32>,
    /// Social Score
    #[serde(rename = "socialScore", skip_serializing_if = "Option::is_none")]
    pub social_score: Option<f32>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl CompanyEsg {
    pub fn new() -> CompanyEsg {
        CompanyEsg {
            symbol: None,
            total_esg_score: None,
            environment_score: None,
            governance_score: None,
            social_score: None,
            data: None,
        }
    }
}


