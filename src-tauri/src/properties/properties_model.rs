use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Property {
    pub id: u32,
    pub title: String,
    pub where_is: String,
    pub characteristics: String,
    pub value: f64,
}
