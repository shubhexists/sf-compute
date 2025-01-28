use crate::order::Order;
use ordered_float::OrderedFloat;
use redis::Commands;
use redis::Connection;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
/// OrderBook struct
/// bids stores buy orders
/// asks stores sell orders
///
/// The structure of the OrderBook is as follows:
/// "PRICE" -> "CLUSTER_SIZE" -> [Order1, Order2, ...]
///
/// THERE IS NESTED BINARY TREE MAPS AS IT PROVIDES O(log n) TIME COMPLEXITY FOR INSERTION AND DELETION, PREVENTS LOOPING THROUGH THE ENTIRE ORDERBOOK
pub struct OrderBook {
    pub bids: BTreeMap<OrderedFloat<f64>, BTreeMap<u64, Vec<Order>>>,
    pub asks: BTreeMap<OrderedFloat<f64>, BTreeMap<u64, Vec<Order>>>,
}

impl OrderBook {
    /// READS ORDERBOOK FROM REDIS and parses it into OrderBook struct
    pub fn load_from_redis(conn: &mut Connection) -> Self {
        let bids: String = conn
            .get("orderbook_bids")
            .unwrap_or_else(|_| "[]".to_string());
        let asks: String = conn
            .get("orderbook_asks")
            .unwrap_or_else(|_| "[]".to_string());

        let bids: BTreeMap<OrderedFloat<f64>, BTreeMap<u64, Vec<Order>>> =
            serde_json::from_str(&bids).unwrap_or_default();
        let asks: BTreeMap<OrderedFloat<f64>, BTreeMap<u64, Vec<Order>>> =
            serde_json::from_str(&asks).unwrap_or_default();

        Self { bids, asks }
    }

    /// SAVES ORDERBOOK TO REDIS
    pub fn save_to_redis(&self, conn: &mut Connection) {
        let bids = serde_json::to_string(&self.bids).unwrap();
        let asks = serde_json::to_string(&self.asks).unwrap();

        let _: () = conn.set("orderbook_bids", bids).unwrap();
        let _: () = conn.set("orderbook_asks", asks).unwrap();
    }

    /// Adds an order to the OrderBook
    pub fn add_order(&mut self, order: Order) -> Option<Order> {
        let target_book = if order.is_buy {
            &mut self.bids
        } else {
            &mut self.asks
        };

        let cluster_map = target_book.entry(OrderedFloat(order.price)).or_default();
        let orders = cluster_map.entry(order.cluster_size).or_default();

        orders.push(order);

        None
    }

    /// Matches an incoming order with an existing order in the OrderBook
    pub fn match_order(&mut self, incoming_order: Order) -> Option<Order> {
        let (target_book, incoming_is_buy) = if incoming_order.is_buy {
            (&mut self.asks, true)
        } else {
            (&mut self.bids, false)
        };

        let mut best_match: Option<(OrderedFloat<f64>, u64, usize)> = None;
        let mut best_price_value = if incoming_is_buy { f64::MAX } else { f64::MIN };

        for (&price, cluster_map) in target_book.iter() {
            if (incoming_is_buy && price > OrderedFloat(incoming_order.price))
                || (!incoming_is_buy && price < OrderedFloat(incoming_order.price))
            {
                break;
            }

            if let Some((&cluster_size, orders)) =
                cluster_map.range(incoming_order.quantity..).next()
            {
                if !orders.is_empty()
                    && ((incoming_is_buy && price.0 < best_price_value)
                        || (!incoming_is_buy && price.0 > best_price_value))
                {
                    best_match = Some((price, cluster_size, 0));
                    best_price_value = price.0;
                }
            }
        }

        if let Some((price, cluster_size, index)) = best_match {
            let matched_order = {
                let cluster_map = target_book.get_mut(&price).unwrap();
                let orders = cluster_map.get_mut(&cluster_size).unwrap();
                let matched_order = &mut orders[index];

                matched_order.quantity -= incoming_order.quantity;

                Order {
                    id: matched_order.id,
                    is_buy: matched_order.is_buy,
                    price: matched_order.price,
                    quantity: incoming_order.quantity,
                    cluster_size: matched_order.cluster_size,
                }
            };

            let cluster_map = target_book.get_mut(&price).unwrap();
            let orders = cluster_map.get_mut(&cluster_size).unwrap();
            if orders[index].quantity == 0 {
                orders.remove(index);
                if orders.is_empty() {
                    cluster_map.remove(&cluster_size);
                    if cluster_map.is_empty() {
                        target_book.remove(&price);
                    }
                }
            }

            return Some(matched_order);
        }

        None
    }

    /// Prints the OrderBook
    pub fn view_orders(&self) {
        println!("\nOrder Book Status:");
        println!("----------------");
        println!("\nBids (Buy Orders):");
        for (price, cluster_map) in self.bids.iter().rev() {
            println!("\nPrice Level: {}", price);
            for (cluster_size, orders) in cluster_map.iter() {
                println!("  Cluster Size {}: {} orders", cluster_size, orders.len());
                for order in orders {
                    println!("    Order ID: {}", order.id);
                    println!("      Quantity: {}", order.quantity);
                    println!("      Is Buy: {}", order.is_buy);
                }
            }
        }

        println!("\nAsks (Sell Orders):");
        for (price, cluster_map) in self.asks.iter() {
            println!("\nPrice Level: {}", price);
            for (cluster_size, orders) in cluster_map.iter() {
                println!("  Cluster Size {}: {} orders", cluster_size, orders.len());
                for order in orders {
                    println!("    Order ID: {}", order.id);
                    println!("      Quantity: {}", order.quantity);
                    println!("      Is Buy: {}", order.is_buy);
                }
            }
        }
    }
}
