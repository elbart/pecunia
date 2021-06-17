use anyhow::{anyhow, Result};
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::{Pool, Postgres};

use crate::{
    client::ApiClient,
    configuration::Configuration,
    database::get_db,
    model::{company::CompanyResponse, intraday_prices::IntradayPrice},
};

pub struct Handler {
    pool: Pool<Postgres>,
    api_client: ApiClient,
    _config: Configuration,
}

impl Handler {
    pub async fn new(config: Configuration) -> Result<Self> {
        Ok(Self {
            pool: get_db(&config).await?,
            api_client: ApiClient::new(&config.iex_api_token),
            _config: config,
        })
    }

    pub async fn get_company(&self, symbol: &String) -> Result<CompanyResponse> {
        let data = self.api_client.get_company(&symbol).await?;

        Ok(data)
    }

    pub async fn get_intraday_prices(&self, symbol: &String) -> Result<Vec<IntradayPrice>> {
        let data = self.api_client.get_intraday_prices(&symbol).await?;
        self.store_intraday_prices(symbol, &data).await?;
        Ok(data)
    }

    pub async fn get_historical_prices(
        &self,
        symbol: &String,
        date: &String,
    ) -> Result<Vec<IntradayPrice>> {
        let data = self.api_client.get_historical_prices(symbol, date).await?;
        self.store_intraday_prices(symbol, &data).await?;
        Ok(data)
    }

    pub async fn get_historical_prices_batch(
        &self,
        symbols: Vec<String>,
        date_from: &String,
        date_to: &String,
    ) -> Result<Vec<Vec<IntradayPrice>>> {
        let mut res = Vec::new();
        let parsed_date_from = NaiveDate::parse_from_str(&date_from, "%Y-%m-%d")?;
        let parsed_date_to = NaiveDate::parse_from_str(&date_to, "%Y-%m-%d")?;
        if parsed_date_from >= parsed_date_to {
            return Err(anyhow!(
                "{:?} is bigger or equals to {:?}",
                parsed_date_from,
                parsed_date_to
            ));
        }

        let mut current_date = parsed_date_from.clone();
        while current_date <= parsed_date_to {
            for symbol in &symbols {
                let data = self
                    .api_client
                    .get_historical_prices(symbol, &current_date.format("%Y%m%d").to_string())
                    .await?;

                self.store_intraday_prices(symbol, &data).await?;
                res.push(data);
            }

            current_date += chrono::Duration::days(1);
        }
        Ok(res)
    }

    async fn store_intraday_prices(
        &self,
        symbol: &String,
        items: &Vec<IntradayPrice>,
    ) -> Result<()> {
        for p in items {
            let time = NaiveDateTime::parse_from_str(
                &format!("{} {}", &p.date, &p.minute),
                "%Y-%m-%d %H:%M",
            )?;

            sqlx::query!(
                r#"INSERT INTO intraday_prices (
                    time,
                    ticker,
                    high,
                    low,
                    "open",
                    "close",
                    average,
                    volume,
                    notional,
                    number_of_trades,
                    change_over_time)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                ON CONFLICT ("time", ticker) DO NOTHING"#,
                time,
                symbol,
                p.high,
                p.low,
                p.open,
                p.close,
                p.average,
                p.volume.and_then(|x| Some(x as f32)),
                p.notional,
                p.number_of_trades as i32,
                p.change_over_time
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }
}
