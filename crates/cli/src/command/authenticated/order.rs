use crate::utils::get_config;
use bpx_api_client::BpxClient;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Order {}

impl Order {
    pub async fn run(&self) -> anyhow::Result<()> {
        todo!()
    }
}
