use crate::utils::get_config;
use bpx_api_client::BpxClient;
use structopt::StructOpt;

pub mod capital;
pub mod history;
pub mod order;

#[derive(Debug, StructOpt)]
pub enum Authenticated {
    Capital(capital::Captial),
    History(history::History),
    Order(order::Order),
}

impl Authenticated {
    pub async fn run(&self) -> anyhow::Result<()> {
        match self {
            Authenticated::Capital(capital) => capital.run().await,
            Authenticated::History(history) => history.run().await,
            Authenticated::Order(order) => order.run().await,
        }
    }
}
