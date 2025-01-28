use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    pub id: u64,
    pub is_buy: bool,
    pub price: f64,
    pub quantity: u64,
    pub cluster_size: u64,
}

impl Order {
    pub fn new(id: u64, is_buy: bool, price: f64, quantity: u64, cluster_size: u64) -> Self {
        Self {
            id,
            is_buy,
            price,
            quantity,
            cluster_size,
        }
    }
}
