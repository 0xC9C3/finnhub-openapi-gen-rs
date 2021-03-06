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
pub struct RedditSentimentContent {
    /// Number of mentions
    #[serde(rename = "mention", skip_serializing_if = "Option::is_none")]
    pub mention: Option<i64>,
    /// Number of positive mentions
    #[serde(rename = "positiveMention", skip_serializing_if = "Option::is_none")]
    pub positive_mention: Option<i64>,
    /// Number of negative mentions
    #[serde(rename = "negativeMention", skip_serializing_if = "Option::is_none")]
    pub negative_mention: Option<i64>,
    /// Positive score. Range 0-1
    #[serde(rename = "positiveScore", skip_serializing_if = "Option::is_none")]
    pub positive_score: Option<f32>,
    /// Negative score. Range 0-1
    #[serde(rename = "negativeScore", skip_serializing_if = "Option::is_none")]
    pub negative_score: Option<f32>,
    /// Final score. Range: -1 to 1 with 1 is very positive and -1 is very negative
    #[serde(rename = "score", skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    /// Period.
    #[serde(rename = "atTime", skip_serializing_if = "Option::is_none")]
    pub at_time: Option<String>,
}

impl RedditSentimentContent {
    pub fn new() -> RedditSentimentContent {
        RedditSentimentContent {
            mention: None,
            positive_mention: None,
            negative_mention: None,
            positive_score: None,
            negative_score: None,
            score: None,
            at_time: None,
        }
    }
}


