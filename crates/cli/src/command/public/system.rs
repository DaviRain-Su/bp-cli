use crate::utils::get_config;
use bpx_api_client::BpxClient;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct System {}

impl System {
    pub async fn run(&self) -> anyhow::Result<()> {
        let config = get_config()?;
        let bpx_client = BpxClient::init(config.base_url, &config.api_key, &config.api_secret)?;

        let v = bpx_client.status().await?;
        println!("status: {:?}", v);

        let v = bpx_client.ping().await?;
        println!("ping: {:?}", v);

        let v = bpx_client.time().await?;
        println!("time: {:?}", v);

        Ok(())
    }
}
