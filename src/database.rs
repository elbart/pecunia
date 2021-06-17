use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::configuration::Configuration;
use anyhow::Result;

pub async fn get_db(cfg: &Configuration) -> Result<PgPool> {
    Ok(PgPoolOptions::new().connect(&cfg.database.url).await?)
}
