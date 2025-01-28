mod order;
mod orderbook;
use clap::{Parser, Subcommand};
use order::Order;
use orderbook::OrderBook;

#[derive(Parser)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Buy {
        #[clap(long, short)]
        price: f64,
        #[clap(long, short)]
        quantity: u64,
        #[clap(long, short)]
        cluster_size: u64,
    },
    Sell {
        #[clap(long, short)]
        price: f64,
        #[clap(long, short)]
        quantity: u64,
        #[clap(long, short)]
        cluster_size: u64,
    },
    View,
}

fn main() {
    let client = redis::Client::open("redis://127.0.0.1:6379");
    match client {
        Ok(client) => {
            println!("Connected to Redis");
            let mut conn = client.get_connection().unwrap();
            let args = CLI::parse();

            let mut order_book = OrderBook::load_from_redis(&mut conn);

            match args.command {
                Commands::Buy {
                    price,
                    quantity,
                    cluster_size,
                } => {
                    println!(
                        "Placing Buy order: price: {}, quantity: {}, cluster_size: {}",
                        price, quantity, cluster_size
                    );

                    let order = Order::new(0, true, price, quantity, cluster_size);
                    match order_book.match_order(order.clone()) {
                        Some(matched_order) => {
                            println!("Matched order: {:?}", matched_order);
                        }
                        None => {
                            order_book.add_order(order);
                        }
                    }

                    order_book.save_to_redis(&mut conn);
                    println!("Order book saved to Redis");
                }

                Commands::Sell {
                    price,
                    quantity,
                    cluster_size,
                } => {
                    println!(
                        "Placing Sell order: price: {}, quantity: {}, cluster_size: {}",
                        price, quantity, cluster_size
                    );

                    let order = Order::new(0, false, price, quantity, cluster_size);
                    match order_book.match_order(order.clone()) {
                        Some(matched_order) => {
                            println!("Matched order: {:?}", matched_order);
                        }
                        None => {
                            order_book.add_order(order);
                        }
                    }

                    order_book.save_to_redis(&mut conn);
                    println!("Order book saved to Redis");
                }

                Commands::View => {
                    println!("Viewing orders");
                    order_book.view_orders();
                }
            }
        }
        Err(e) => {
            eprintln!("Error connecting to Redis: {}", e);
            std::process::exit(1);
        }
    }
}
