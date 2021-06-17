use crate::model::{company::CompanyResponse, intraday_prices::PricesResponse};
use anyhow::Result;
use log::debug;
use reqwest::{header::HeaderMap, Client, Method};
use serde::{de::DeserializeOwned, Serialize};

const BASE_URL: &'static str = "https://cloud.iexapis.com/stable";

pub struct ApiClient {
    base_url: String,
    api_token: String,
    http_client: Client,
}

#[derive(Debug, Serialize)]
struct IexCloudResponseHeaders {
    iexcloud_messages_used: usize,
    iexcloud_credits_used: usize,
    iexcloud_premium_message_used: usize,
    iexcloud_premium_credits_used: usize,
}

impl From<HeaderMap> for IexCloudResponseHeaders {
    fn from(header: HeaderMap) -> Self {
        Self {
            iexcloud_messages_used: header.get("iexcloud-messages-used").map_or(0, |value| {
                value
                    .to_str()
                    .map_or(0, |data| data.to_string().parse().unwrap_or(0))
            }),
            iexcloud_credits_used: header.get("iexcloud-credits-used").map_or(0, |value| {
                value
                    .to_str()
                    .map_or(0, |data| data.to_string().parse().unwrap_or(0))
            }),
            iexcloud_premium_message_used: header.get("iexcloud-premium-messages-used").map_or(
                0,
                |value| {
                    value
                        .to_str()
                        .map_or(0, |data| data.to_string().parse().unwrap_or(0))
                },
            ),
            iexcloud_premium_credits_used: header.get("iexcloud-premium-credits-used").map_or(
                0,
                |value| {
                    value
                        .to_str()
                        .map_or(0, |data| data.to_string().parse().unwrap_or(0))
                },
            ),
        }
    }
}

impl ApiClient {
    /// Creates a new API client.
    pub fn new(api_token: &String) -> Self {
        Self {
            base_url: BASE_URL.to_string(),
            api_token: api_token.clone(),
            http_client: Client::new(),
        }
    }

    /// Executes the actual HTTP request.
    async fn execute_request<T>(&self, method: reqwest::Method, url: String) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self
            .http_client
            .request(method.clone(), &url)
            .query(&[("token", &self.api_token)])
            .send()
            .await?;

        let iex_headers = IexCloudResponseHeaders::from(response.headers().clone());
        let json_body: serde_json::Value = response.json().await?;

        debug!(
            "Response data for request {} {} are: IEX Headers: {}; JSON Body: {}",
            method.as_str(),
            &url,
            &serde_json::to_string(&iex_headers)?,
            &serde_json::to_string(&json_body)?
        );

        Ok(serde_json::from_value(json_body)?)
    }

    /// Fetches Company information for the specified symbol.
    /// Docs are here: https://iexcloud.io/docs/api/#company.
    pub async fn get_company(&self, symbol: &String) -> Result<CompanyResponse> {
        let url = format!("{}/stock/{}/company", self.base_url, symbol);

        Ok(self.execute_request(Method::GET, url).await?)
    }

    /// Fetches current intraday prices.
    /// Docs are here: https://iexcloud.io/docs/api/#intraday-prices.
    pub async fn get_intraday_prices(&self, symbol: &String) -> Result<PricesResponse> {
        let url = format!("{}/stock/{}/intraday-prices", self.base_url, symbol);

        Ok(self.execute_request(Method::GET, url).await?)
    }

    /// Fetches historical intraday prices for the specified date.
    /// Docs are here: https://iexcloud.io/docs/api/#historical-prices.
    pub async fn get_historical_prices(
        &self,
        symbol: &String,
        date: &String,
    ) -> Result<PricesResponse> {
        let url = format!("{}/stock/{}/chart/date/{}", self.base_url, symbol, date);

        Ok(self.execute_request(Method::GET, url).await?)
    }
}
