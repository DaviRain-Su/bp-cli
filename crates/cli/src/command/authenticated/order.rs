use crate::utils::get_config;
use bpx_api_client::BpxClient;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Order {}

impl Order {
    pub async fn run(&self) -> anyhow::Result<()> {
        let config = get_config()?;
        let bpx_client = BpxClient::init(config.base_url, &config.api_key, &config.api_secret)?;

        let open_orders = bpx_client.get_open_orders(Some("SOL_USDC")).await?;
        println!("open orders: {:?}", open_orders);

        // let open_order = bpx_client.get_open_order("SOL_USDC", None, None).await?;
        // println!("open order: {:?}", open_order);

        Ok(())
    }
}
