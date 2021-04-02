use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RotkiAll {
    #[serde(rename = "type")]
    pub action: String,
    pub location: String,
    pub paid_asset: String,
    pub paid_in_asset: Option<String>,
    pub taxable_amount: Option<String>,
    pub received_asset: Option<String>,
    pub received_in_asset: Option<String>,
    pub net_profit_or_loss: Option<String>,
    pub time: Option<String>,
    pub is_virtual: Option<String>,
    pub paid_in_EUR: Option<String>,
    pub taxable_received_in_EUR: Option<String>,
    pub taxable_bought_cost_in_EUR: Option<String>,
    pub cost_basis: Option<String>,
    pub total_bought_cost_in_EUR: Option<String>,
    pub total_received_in_EUR: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AssetMovements {
    #[serde(rename = "type")]
    pub action: String,
    pub exchange: String,
    pub moving_asset: String,
    pub fee_in_asset: Option<String>,
    pub fee_in_EUR: Option<String>,
    pub time: Option<String>,
}
#[derive(Debug, Default)]
pub struct DirType{
    pub name: String,
    pub isDir: bool,
}