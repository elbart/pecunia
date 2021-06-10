use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct CompanyResponse {
    symbol: String,
    company_name: String,
    exchange: String,
    industry: String,
    website: String,
    description: String,
    #[serde(alias = "CEO")]
    ceo: String,
    security_name: String,
    issue_type: String,
    sector: String,
    primary_sic_code: Option<usize>,
    employees: Option<usize>,
    tags: Vec<String>,
    address: Option<String>,
    address2: Option<String>,
    state: Option<String>,
    city: Option<String>,
    zip: Option<String>,
    country: Option<String>,
    phone: Option<String>,
}
