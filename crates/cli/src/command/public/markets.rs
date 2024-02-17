use crate::utils::get_config;
use bpx_api_client::BpxClient;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Market {}

impl Market {
    pub async fn run(&self) -> anyhow::Result<()> {
        let config = get_config()?;
        let bpx_client = BpxClient::init(config.base_url, &config.api_key, &config.api_secret)?;

        let asset = bpx_client.get_assets().await?;
        println!("asset: {:?}", asset);

        let market = bpx_client.get_markets().await?;
        println!("market: {:?}", market);

        let tickets = bpx_client.get_ticker("SOL_USDC").await?;
        println!("tickets: {:#?}", tickets);

        let order_book_depth = bpx_client.get_order_book_depth("SOL_USDC").await?;
        println!("order_book_depth: {:?}", order_book_depth);

        Ok(())
    }
}
