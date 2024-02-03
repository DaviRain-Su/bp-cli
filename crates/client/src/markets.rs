use bpx_api_types::markets::{Kline, Market, OrderBookDepth, Ticker, TokenList};

use crate::error::Result;
use crate::BpxClient;

impl BpxClient {
    /// Get a list of all available assets.
    pub async fn get_assets(&self) -> Result<Vec<TokenList>> {
        let url = format!("{}/api/v1/assets", self.base_url);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Get a list of all available markets.
    pub async fn get_markets(&self) -> Result<Vec<Market>> {
        let url = format!("{}/api/v1/markets", self.base_url);
        log::info!("url: {:?}", url);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    pub async fn get_ticker(&self, symbol: &str) -> Result<Vec<Ticker>> {
        let url = format!("{}/api/v1/ticker&symbol={}", self.base_url, symbol);
        log::info!("url: {:?}", url);
        let res = self.get(url).await?;
        // log::info!("res: {:?}", res);
        let text = res.text().await?;
        log::info!("text: {:?}", text);
        // res.json().await.map_err(Into::into)
        todo!()
    }

    pub async fn get_order_book_depth(&self, symbol: &str) -> Result<OrderBookDepth> {
        let url = format!("{}/api/v1/depth&symbol={}", self.base_url, symbol);
        log::info!("url: {:?}", url);
        let res = self.get(url).await?;
        let text = res.text().await?;
        log::info!("text: {:?}", text);
        // res.json().await.map_err(Into::into)
        todo!()
    }

    pub async fn get_k_lines(
        &self,
        symbol: &str,
        kline_interval: &str,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<Vec<Kline>> {
        let mut url = format!(
            "/{}/api/v1/klines?symbol={}&kline_interval={}",
            self.base_url, symbol, kline_interval
        );
        for (k, v) in [("start_time", start_time), ("end_time", end_time)] {
            if let Some(v) = v {
                url.push_str(&format!("&{}={}", k, v));
            }
        }
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }
}