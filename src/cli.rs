use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Pecunia CLI client",
    about = "Command line interface for interaction with IEX Cloud"
)]
pub struct PecuniaCli {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    Get(Resource),
}

/// Retrieve data for a specific resource.
#[derive(Debug, StructOpt)]
pub enum Resource {
    /// Fetch general company information.
    Company {
        /// Stock Symbol e.g. AAPL
        symbol: String,
    },
    /// Fetch intraday prices.
    IntradayPrices {
        /// Stock symbol e.g. AAPL
        symbol: String,
    },
    /// Fetch historical intraday prices.
    HistoricalPrices {
        /// Stock symbol e.g. AAPL
        symbol: String,
        /// Date in format YYYYMMDD e.g. 20210521
        date: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationData {
    pub api_token: String,
}
