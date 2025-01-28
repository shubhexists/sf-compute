use clap::{Parser, Subcommand};

#[derive(Parser)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Place {
        #[clap(long, short)]
        is_buy: bool,
        #[clap(long, short)]
        price: f64,
        #[clap(long, short)]
        quantity: u64,
        #[clap(long, short)]
        cluster_size: u64,
    },
    Match {
        #[clap(long, short)]
        id: u64,
    },
    View,
}

fn main() {
    let args = CLI::parse();
    match args.command {
        Commands::Place {
            is_buy,
            price,
            quantity,
            cluster_size,
        } => {
            println!(
                "Placing order: is_buy: {}, price: {}, quantity: {}, cluster_size: {}",
                is_buy, price, quantity, cluster_size
            );
        }
        Commands::Match { id } => {
            println!("Matching order: id: {}", id);
        }
        Commands::View => {
            println!("Viewing orders");
        }
    }
}
