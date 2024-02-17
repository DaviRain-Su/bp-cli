use crate::utils::get_config;
use bpx_api_client::BpxClient;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Trades {}

impl Trades {
    pub async fn run(&self) -> anyhow::Result<()> {
        let config = get_config()?;
        let bpx_client = BpxClient::init(config.base_url, &config.api_key, &config.api_secret)?;

        let trade = bpx_client.get_recent_trades("SOL_USDC", None).await?;
        println!("trade: {:?}", trade);

        let history = bpx_client
            .get_historical_trades("SOL_USDC", None, None)
            .await?;
        println!("history: {:?}", history);
        Ok(())
    }
}
