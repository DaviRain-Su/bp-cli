use crate::utils::get_config;
use bpx_api_client::BpxClient;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Market {}

impl Market {
    pub async fn run(&self) -> anyhow::Result<()> {
        todo!()
    }
}
