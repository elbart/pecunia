use anyhow::Result;
use log::{self, debug, info};
use pecunia::{configuration, handler::Handler};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Pecunia CLI client",
    about = "Command line interface for interaction with IEX Cloud"
)]
pub struct PecuniaCli {
    #[structopt(subcommand)]
    pub cmd: Command,

    /// Wether to persist a fetched entry or not
    #[structopt(long)]
    pub persist: bool,
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

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init_custom_env("PECUNIA_LOG");
    let opt = PecuniaCli::from_args();

    debug!("Given command / subcommand + arguments are: {:?}", opt);

    let cfg = configuration::Configuration::new().unwrap();
    let handlers = Handler::new(cfg).await?;

    match opt.cmd {
        Command::Get(c) => {
            match c {
                Resource::Company { symbol } => {
                    info!("Got subcommand 'get company'. Fetching company information ...");
                    let data = handlers.get_company(&symbol).await?;
                    println!("{}", serde_json::to_string_pretty(&data)?);
                }
                Resource::IntradayPrices { symbol } => {
                    info!("Got subcommand 'get intraday-prices'. Fetching intraday price information ...");
                    let data = handlers.get_intraday_prices(&symbol).await?;
                    println!("{}", serde_json::to_string_pretty(&data)?);
                }
                Resource::HistoricalPrices { symbol, date } => {
                    info!("Got subcommand 'get historical-prices'. Fetching historical price information ...");
                    let data = handlers.get_historical_prices(&symbol, &date).await?;
                    println!("{}", serde_json::to_string_pretty(&data)?);
                }
            }
        }
    }

    Ok(())
}
