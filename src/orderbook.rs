use crate::order::Order;
use ordered_float::OrderedFloat;
use redis::Commands;
use redis::Connection;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
pub struct OrderBook {
    pub bids: BTreeMap<OrderedFloat<f64>, Vec<Order>>,
    pub asks: BTreeMap<OrderedFloat<f64>, Vec<Order>>,
}

impl OrderBook {
    pub fn load_from_redis(conn: &mut Connection) -> Self {
        let bids: String = conn
            .get("orderbook_bids")
            .unwrap_or_else(|_| "[]".to_string());
        let asks: String = conn
            .get("orderbook_asks")
            .unwrap_or_else(|_| "[]".to_string());

        let bids: BTreeMap<OrderedFloat<f64>, Vec<Order>> =
            serde_json::from_str(&bids).unwrap_or_default();
        let asks: BTreeMap<OrderedFloat<f64>, Vec<Order>> =
            serde_json::from_str(&asks).unwrap_or_default();

        Self { bids, asks }
    }

    pub fn save_to_redis(&self, conn: &mut Connection) {
        let bids = serde_json::to_string(&self.bids).unwrap();
        let asks = serde_json::to_string(&self.asks).unwrap();

        let _: () = conn.set("orderbook_bids", bids).unwrap();
        let _: () = conn.set("orderbook_asks", asks).unwrap();
    }

    pub fn add_order(&mut self, order: Order) -> Option<Order> {
        let target_book = if order.is_buy {
            &mut self.bids
        } else {
            &mut self.asks
        };

        target_book
            .entry(OrderedFloat(order.price))
            .or_default()
            .push(order);

        None
    }

    pub fn match_order(&mut self, incoming_order: Order) -> Option<Order> {
        let (target_book, incoming_is_buy) = if incoming_order.is_buy {
            (&mut self.asks, true)
        } else {
            (&mut self.bids, false)
        };

        let mut matched_price = None;
        let mut matched_index = None;

        for (&price, orders) in target_book.iter() {
            if (incoming_is_buy && price > OrderedFloat(incoming_order.price))
                || (!incoming_is_buy && price < OrderedFloat(incoming_order.price))
            {
                break;
            }

            if let Some(pos) = orders
                .iter()
                .position(|o| o.cluster_size >= incoming_order.quantity)
            {
                matched_price = Some(price);
                matched_index = Some(pos);
                break;
            }
        }

        if let (Some(price), Some(pos)) = (matched_price, matched_index) {
            let matched_order = {
                let orders = target_book.get_mut(&price).unwrap();
                let matched_order = &mut orders[pos];

                matched_order.quantity -= incoming_order.quantity;

                Order {
                    id: matched_order.id,
                    is_buy: matched_order.is_buy,
                    price: matched_order.price,
                    quantity: incoming_order.quantity,
                    cluster_size: incoming_order.cluster_size,
                }
            };

            if let Some(orders) = target_book.get_mut(&price) {
                if orders[pos].quantity == 0 {
                    orders.remove(pos);
                }
                if orders.is_empty() {
                    target_book.remove(&price);
                }
            }

            return Some(matched_order);
        }

        None
    }

    pub fn view_orders(&self) {
        println!("Bids:");
        for (price, orders) in self.bids.iter() {
            for order in orders {
                println!(
                    "id: {}, is_buy: {}, price: {}, quantity: {}, cluster_size: {}",
                    order.id, order.is_buy, price, order.quantity, order.cluster_size
                );
            }
        }

        println!("Asks:");
        for (price, orders) in self.asks.iter() {
            for order in orders {
                println!(
                    "id: {}, is_buy: {}, price: {}, quantity: {}, cluster_size: {}",
                    order.id, order.is_buy, price, order.quantity, order.cluster_size
                );
            }
        }
    }
}
