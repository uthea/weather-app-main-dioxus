use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub results: Option<Vec<SearchResult>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub id: i64,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: f64,
    #[serde(rename = "feature_code")]
    pub feature_code: String,
    #[serde(rename = "country_code")]
    pub country_code: String,
    pub timezone: String,
    pub population: Option<i64>,
    #[serde(rename = "country_id")]
    pub country_id: i64,
    pub country: String,
}
