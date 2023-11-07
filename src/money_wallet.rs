use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
pub struct MoneyWallet {
    pub wallet: String,
    pub currency: String,
    pub category: String,
    pub datetime: String,
    pub money: String,
    pub description: String,
}
