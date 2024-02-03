use crate::error::Result;

use crate::BpxClient;
use bpx_api_types::system::Status;

impl BpxClient {
    pub async fn status(&self) -> Result<Status> {
        let url = format!("{}/api/v1/status", self.base_url);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    pub async fn ping(&self) -> Result<String> {
        let url = format!("{}/api/v1/ping", self.base_url);
        let res = self.get(url).await?;
        res.text().await.map_err(Into::into)
    }

    pub async fn time(&self) -> Result<i64> {
        let url = format!("{}/api/v1/time", self.base_url);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }
}
