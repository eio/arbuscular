// Define JSON payloads as strongly typed data structures
use serde::{Serialize, Deserialize};

//////////////
// Components
//////////////
#[derive(Deserialize, Serialize)]
pub struct Coin {
    pub coin: String
}
#[derive(Deserialize, Serialize)]
pub struct Quantity {
    pub coin: String,
    pub amount: f64
}
#[derive(Deserialize, Serialize)]
pub struct Recipient {
    pub address: String
}

///////////////
// API Actions
///////////////
#[derive(Deserialize, Serialize)]
pub struct SendCoin {
    #[serde(with = "serde_with::json::nested")]
    pub quantity: Quantity,
    pub recipient: Recipient
}
#[derive(Deserialize, Serialize)]
pub struct ExchangeCoin {
    #[serde(with = "serde_with::json::nested")]
    pub exchange_this: Quantity,
    pub get_this: Coin
}
#[derive(Deserialize, Serialize)]
pub struct ExchangeReceipt {
    #[serde(with = "serde_with::json::nested")]
    pub this_was_exchanged: Quantity,
    pub got_item: Quantity
}