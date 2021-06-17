use std::convert::TryInto;

use anyhow::Result;
use log::info;
use sqlx::{
    types::chrono::{NaiveDate, NaiveDateTime},
    Pool, Postgres,
};

use crate::{
    client::ApiClient,
    configuration::Configuration,
    database::get_db,
    model::{company::CompanyResponse, intraday_prices::IntradayPrice},
};

pub struct Handler {
    pool: Pool<Postgres>,
    api_client: ApiClient,
    config: Configuration,
}

impl Handler {
    pub async fn new(config: Configuration) -> Result<Self> {
        Ok(Self {
            pool: get_db(&config).await?,
            api_client: ApiClient::new(&config.iex_api_token),
            config,
        })
    }

    pub async fn get_company(&self, symbol: &String) -> Result<CompanyResponse> {
        let data = self.api_client.get_company(&symbol).await?;

        Ok(data)
    }

    pub async fn get_intraday_prices(&self, symbol: &String) -> Result<Vec<IntradayPrice>> {
        let data = self.api_client.get_intraday_prices(&symbol).await?;

        for p in &data {
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
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)"#,
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

        Ok(data)
    }

    pub async fn get_historical_prices(
        &self,
        symbol: &String,
        date: &String,
    ) -> Result<Vec<IntradayPrice>> {
        let data = self.api_client.get_historical_prices(symbol, date).await?;

        Ok(data)
    }
}
