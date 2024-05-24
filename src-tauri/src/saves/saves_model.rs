use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Savings {
    pub id: u32,
    pub where_is: String,
    pub amount: f64,
}
