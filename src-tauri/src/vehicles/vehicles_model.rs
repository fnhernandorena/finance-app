use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Vehicle {
    pub id: u32,
    pub brand: String,
    pub model: String,
    pub year: u32,
    pub value: f64,
}
