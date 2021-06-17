use serde::{Deserialize, Serialize};

pub type PricesResponse = Vec<IntradayPrice>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct IntradayPrice {
    pub date: String,
    pub minute: String,
    pub label: String,
    pub high: Option<f32>,
    pub low: Option<f32>,
    pub open: Option<f32>,
    pub close: Option<f32>,
    pub average: Option<f32>,
    pub volume: Option<usize>,
    pub notional: Option<f32>,
    pub number_of_trades: usize,
    pub change_over_time: Option<f32>,
}
