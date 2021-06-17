use crate::{
    cli::{get_authentication_information, Command, PecuniaCli, Resource},
    client::ApiClient,
};
use anyhow::Result;
use log::{self, debug, info};
use structopt::StructOpt;

mod cli;
mod client;
mod configuration;
mod model;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init_custom_env("PECUNIA_LOG");
    let opt = PecuniaCli::from_args();

    debug!("Given command / subcommand + arguments are: {:?}", opt);

    let cfg = configuration::Configuration::new().unwrap();

    match opt.cmd {
        Command::Get(c) => match c {
            Resource::Company { symbol } => {
                info!("Got subcommand 'get company'. Fetching company information ...");
                let client = ApiClient::new(cfg.iex_api_token);
                println!(
                    "{}",
                    serde_json::to_string_pretty(&client.get_company(&symbol).await?)?
                );
            }
            Resource::IntradayPrices { symbol } => {
                info!("Got subcommand 'get intraday-prices'. Fetching intraday price information information ...");
                let client = ApiClient::new(cfg.iex_api_token);
                println!(
                    "{}",
                    serde_json::to_string_pretty(&client.get_intraday_prices(&symbol).await?)?
                );
            }
            Resource::HistoricalPrices { symbol, date } => {
                info!("Got subcommand 'get historical-prices'. Fetching historical price information information ...");
                let client = ApiClient::new(cfg.iex_api_token);
                println!(
                    "{}",
                    serde_json::to_string_pretty(
                        &client.get_historical_prices(&symbol, date).await?
                    )?
                );
            }
        },
    }

    Ok(())
}
