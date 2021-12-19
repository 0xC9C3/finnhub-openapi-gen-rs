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
pub struct CompanyProfile {
    /// Address of company's headquarter.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// City of company's headquarter.
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Country of company's headquarter.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Currency used in company filings.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// CUSIP number.
    #[serde(rename = "cusip", skip_serializing_if = "Option::is_none")]
    pub cusip: Option<String>,
    /// Sedol number.
    #[serde(rename = "sedol", skip_serializing_if = "Option::is_none")]
    pub sedol: Option<String>,
    /// Company business summary.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Listed exchange.
    #[serde(rename = "exchange", skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    /// Industry group.
    #[serde(rename = "ggroup", skip_serializing_if = "Option::is_none")]
    pub ggroup: Option<String>,
    /// Industry.
    #[serde(rename = "gind", skip_serializing_if = "Option::is_none")]
    pub gind: Option<String>,
    /// Sector.
    #[serde(rename = "gsector", skip_serializing_if = "Option::is_none")]
    pub gsector: Option<String>,
    /// Sub-industry.
    #[serde(rename = "gsubind", skip_serializing_if = "Option::is_none")]
    pub gsubind: Option<String>,
    /// ISIN number.
    #[serde(rename = "isin", skip_serializing_if = "Option::is_none")]
    pub isin: Option<String>,
    /// NAICS national industry.
    #[serde(rename = "naicsNationalIndustry", skip_serializing_if = "Option::is_none")]
    pub naics_national_industry: Option<String>,
    /// NAICS industry.
    #[serde(rename = "naics", skip_serializing_if = "Option::is_none")]
    pub naics: Option<String>,
    /// NAICS sector.
    #[serde(rename = "naicsSector", skip_serializing_if = "Option::is_none")]
    pub naics_sector: Option<String>,
    /// NAICS subsector.
    #[serde(rename = "naicsSubsector", skip_serializing_if = "Option::is_none")]
    pub naics_subsector: Option<String>,
    /// Company name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Company phone number.
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// State of company's headquarter.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Company symbol/ticker as used on the listed exchange.
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// Company website.
    #[serde(rename = "weburl", skip_serializing_if = "Option::is_none")]
    pub weburl: Option<String>,
    /// IPO date.
    #[serde(rename = "ipo", skip_serializing_if = "Option::is_none")]
    pub ipo: Option<String>,
    /// Market Capitalization.
    #[serde(rename = "marketCapitalization", skip_serializing_if = "Option::is_none")]
    pub market_capitalization: Option<f32>,
    /// Number of oustanding shares.
    #[serde(rename = "shareOutstanding", skip_serializing_if = "Option::is_none")]
    pub share_outstanding: Option<f32>,
    /// Number of employee.
    #[serde(rename = "employeeTotal", skip_serializing_if = "Option::is_none")]
    pub employee_total: Option<f32>,
    /// Logo image.
    #[serde(rename = "logo", skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    /// Finnhub industry classification.
    #[serde(rename = "finnhubIndustry", skip_serializing_if = "Option::is_none")]
    pub finnhub_industry: Option<String>,
}

impl CompanyProfile {
    pub fn new() -> CompanyProfile {
        CompanyProfile {
            address: None,
            city: None,
            country: None,
            currency: None,
            cusip: None,
            sedol: None,
            description: None,
            exchange: None,
            ggroup: None,
            gind: None,
            gsector: None,
            gsubind: None,
            isin: None,
            naics_national_industry: None,
            naics: None,
            naics_sector: None,
            naics_subsector: None,
            name: None,
            phone: None,
            state: None,
            ticker: None,
            weburl: None,
            ipo: None,
            market_capitalization: None,
            share_outstanding: None,
            employee_total: None,
            logo: None,
            finnhub_industry: None,
        }
    }
}

