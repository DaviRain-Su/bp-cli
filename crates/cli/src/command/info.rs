// use crate::utils::get_config;
// use bpx_api_client::BpxClient;
// use structopt::StructOpt;

// #[derive(Debug, StructOpt)]
// pub struct Info {}

// impl Info {
//     pub async fn run(&self) -> anyhow::Result<()> {
//         let config = get_config()?;

//         let bpx_client = BpxClient::init(config.base_url, &config.api_key, &config.api_secret)?;

//         let asserts = bpx_client.get_assets().await?;
//         println!("Asserts: {:#?}", asserts);
//         //
//         // let market = bpx_client.get_markets().await?;
//         // println!("Market: {:#?}", market);

//         // let ticks = bpx_client.get_ticker("SOL").await?;
//         // println!("Ticks: {:#?}", ticks);

//         // let balance = bpx_client.get_balances().await?;
//         // println!("Balance: {:#?}", balance);

//         // let deposit_address = bpx_client
//         // .get_deposit_address(bpx_api_client::types::Blockchain::Solana)
//         // .await?;
//         // println!("DepositAddress: {:#?}", deposit_address);

//         // let deposits = bpx_client.get_deposits(None, None).await?;
//         // println!("Deposits: {:#?}", deposits);

//         // let withdraws = bpx_client.get_withdrawals(None, None).await?;
//         // println!("Withdraws: {:#?}", withdraws);

//         // let recent_trade = bpx_client.get_recent_trades("SOL_USDC", None).await?;
//         // println!("RecentTrade: {:#?}", recent_trade);

//         // let historical_trade = bpx_client
//         // .get_historical_trades("SOL_USDC", None, None)
//         // .await?;
//         // println!("HistoricalTrade: {:#?}", historical_trade);

//         // let open_order = bpx_client.get_open_orders(Some("SOL_USDC")).await?;
//         // println!("OpenOrder: {:#?}", open_order);

//         let depth = bpx_client.get_order_book_depth("SOL_USDC").await?;
//         println!("Depth: {:#?}", depth);

//         // let status = bpx_client.status().await?;
//         // println!("Status: {:#?}", status);

//         // let ping = bpx_client.ping().await?;
//         // println!("Ping: {:#?}", ping);

//         // let time = bpx_client.time().await?;
//         // println!("Time: {:#?}", time);
//         Ok(())
//     }
// }
