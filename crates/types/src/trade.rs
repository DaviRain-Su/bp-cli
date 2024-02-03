use std::fmt;
use std::fmt::Debug;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: i64,
    pub price: Decimal,
    pub quantity: Decimal,
    pub quote_quantity: Decimal,
    pub timestamp: i64,
    pub is_buyer_maker: bool,
}

impl Debug for Trade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Trade")
            .field("id", &self.id)
            .field("price", &self.price)
            .field("quantity", &self.quantity)
            .field("quote_quantity", &self.quote_quantity)
            .field("timestamp", &self.timestamp)
            .field("is_buyer_maker", &self.is_buyer_maker)
            .finish()
    }
}
