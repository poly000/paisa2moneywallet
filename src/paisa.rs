use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaisaJson {
    pub version: i64,
    pub expenses: Vec<Expense>,
    pub accounts: Vec<Account>,
    pub categories: Vec<Category>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Expense {
    pub name: String,
    /// need workaround here: add quote to Paisa currency,
    /// thus we could get accurate number easily
    pub currency: String,
    pub time: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub account_id: usize,
    pub category_id: usize,
    pub super_id: u64,
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub name: String,
    pub bank_name: String,
    pub number: String,
    pub card_type: String,
    pub super_id: usize,
    pub amount: f64,
    pub color: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub name: String,
    pub description: Option<String>,
    pub icon: i64,
    pub super_id: i64,
    pub budget: f64,
    pub color: i64,
    pub is_budget: bool,
    pub is_default: bool,
}
