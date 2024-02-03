use structopt::StructOpt;

pub mod command;
pub mod config;
pub mod constant;
pub mod error;
pub mod utils;

#[derive(Debug, StructOpt)]
pub enum BpCli {
    Auto(command::auto::Auto),
    Info(command::info::Info),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let opt = BpCli::from_args();
    match opt {
        BpCli::Auto(auto) => {
            auto.run()?;
        }
        BpCli::Info(info) => {
            info.run().await?;
        }
    }
    Ok(())
}
