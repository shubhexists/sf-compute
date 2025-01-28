use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Order struct
/// ```rust
/// Order {
///    id: 0, // This is the order id
///    is_buy: true, // true if buy order, false if sell order
///    price: 0.0,  // price of the order
///    quantity: 0, // quantity of the order
///    cluster_size: 0, // cluster size of the order
/// }
/// ```
pub struct Order {
    pub id: u64,
    pub is_buy: bool,
    pub price: f64,
    pub quantity: u64,
    pub cluster_size: u64,
}

impl Order {
    /// Creates a new Order
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
