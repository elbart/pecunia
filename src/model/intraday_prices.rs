use serde::{Deserialize, Serialize};

pub type PricesResponse = Vec<IntradayPrice>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct IntradayPrice {
    date: String,
    minute: String,
    label: String,
    high: Option<f32>,
    low: Option<f32>,
    open: Option<f32>,
    close: Option<f32>,
    average: Option<f32>,
    volume: Option<usize>,
    notional: Option<f32>,
    number_of_trades: usize,
    change_over_time: Option<f32>,
}
