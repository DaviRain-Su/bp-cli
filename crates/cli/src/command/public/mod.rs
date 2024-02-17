use crate::utils::get_config;
use bpx_api_client::BpxClient;
use structopt::StructOpt;

pub mod markets;
pub mod system;
pub mod trades;

#[derive(Debug, StructOpt)]
pub enum Public {
    Market(markets::Market),
    System(system::System),
    Trades(trades::Trades),
}

impl Public {
    pub async fn run(&self) -> anyhow::Result<()> {
        match self {
            Public::Market(market) => market.run().await,
            Public::System(system) => system.run().await,
            Public::Trades(trades) => trades.run().await,
        }
    }
}
