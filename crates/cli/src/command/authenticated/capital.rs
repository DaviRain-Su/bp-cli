use crate::utils::get_config;
use bpx_api_client::BpxClient;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Captial {}

impl Captial {
    pub async fn run(&self) -> anyhow::Result<()> {
        let config = get_config()?;
        let bpx_client = BpxClient::init(config.base_url, &config.api_key, &config.api_secret)?;

        let balance = bpx_client.get_balances().await?;
        println!("balance is {:#?}", balance);

        // let result = bpx_client.get_deposits(None, None).await?;
        // println!("deposit result: {:#?}", result);

        // let result = bpx_client
        //     .get_deposit_address(bpx_api_client::types::Blockchain::Solana)
        //     .await?;
        // println!("deposit result: {:#?}", result);

        // let result = bpx_client.get_withdrawals(None, None).await?;
        // println!("withdrawals result: {:#?}", result);

        // todo
        // request_withdrawal
        Ok(())
    }
}
