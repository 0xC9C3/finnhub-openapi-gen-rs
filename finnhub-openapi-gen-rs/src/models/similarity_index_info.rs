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
pub struct SimilarityIndexInfo {
    /// CIK.
    #[serde(rename = "cik", skip_serializing_if = "Option::is_none")]
    pub cik: Option<String>,
    /// Cosine similarity of Item 1 (Business). This number is only available for Annual reports.
    #[serde(rename = "item1", skip_serializing_if = "Option::is_none")]
    pub item1: Option<f32>,
    /// Cosine similarity of Item 1A (Risk Factors). This number is available for both Annual and Quarterly reports.
    #[serde(rename = "item1a", skip_serializing_if = "Option::is_none")]
    pub item1a: Option<f32>,
    /// Cosine similarity of Item 2 (Management’s Discussion and Analysis of Financial Condition and Results of Operations). This number is only available for Quarterly reports.
    #[serde(rename = "item2", skip_serializing_if = "Option::is_none")]
    pub item2: Option<f32>,
    /// Cosine similarity of Item 7 (Management’s Discussion and Analysis of Financial Condition and Results of Operations). This number is only available for Annual reports.
    #[serde(rename = "item7", skip_serializing_if = "Option::is_none")]
    pub item7: Option<f32>,
    /// Cosine similarity of Item 7A (Quantitative and Qualitative Disclosures About Market Risk). This number is only available for Annual reports.
    #[serde(rename = "item7a", skip_serializing_if = "Option::is_none")]
    pub item7a: Option<f32>,
    /// Access number.
    #[serde(rename = "accessNumber", skip_serializing_if = "Option::is_none")]
    pub access_number: Option<String>,
    /// Form type.
    #[serde(rename = "form", skip_serializing_if = "Option::is_none")]
    pub form: Option<String>,
    /// Filed date <code>%Y-%m-%d %H:%M:%S</code>.
    #[serde(rename = "filedDate", skip_serializing_if = "Option::is_none")]
    pub filed_date: Option<String>,
    /// Accepted date <code>%Y-%m-%d %H:%M:%S</code>.
    #[serde(rename = "acceptedDate", skip_serializing_if = "Option::is_none")]
    pub accepted_date: Option<String>,
    /// Report's URL.
    #[serde(rename = "reportUrl", skip_serializing_if = "Option::is_none")]
    pub report_url: Option<String>,
    /// Filing's URL.
    #[serde(rename = "filingUrl", skip_serializing_if = "Option::is_none")]
    pub filing_url: Option<String>,
}

impl SimilarityIndexInfo {
    pub fn new() -> SimilarityIndexInfo {
        SimilarityIndexInfo {
            cik: None,
            item1: None,
            item1a: None,
            item2: None,
            item7: None,
            item7a: None,
            access_number: None,
            form: None,
            filed_date: None,
            accepted_date: None,
            report_url: None,
            filing_url: None,
        }
    }
}

