mod order;
mod order_book;
mod engine;

use std::io::{self, Write};
use order::{OrderSide, OrderType};
use engine::MatchingEngine;

fn main() {
    let mut engine = MatchingEngine::new();
    println!("🚀 Welcome to Rust Trading Engine CLI!");
    println!("Format: order_type side price quantity");
    println!("Example: limit buy 101.5 2.0 OR market sell - 1.5");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("❌ Failed to read input");
            continue;
        }

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() != 4 {
            println!("❗ Invalid input. Expected 4 parts.");
            continue;
        }

        let order_type = match parts[0].to_lowercase().as_str() {
            "limit" => OrderType::Limit,
            "market" => OrderType::Market,
            _ => {
                println!("❗ Unknown order type");
                continue;
            }
        };

        let side = match parts[1].to_lowercase().as_str() {
            "buy" => OrderSide::Buy,
            "sell" => OrderSide::Sell,
            _ => {
                println!("❗ Unknown side");
                continue;
            }
        };

        let price = if order_type == OrderType::Limit {
            match parts[2].parse::<f64>() {
                Ok(p) => Some(p),
                Err(_) => {
                    println!("❗ Invalid price");
                    continue;
                }
            }
        } else {
            None
        };

        let quantity = match parts[3].parse::<f64>() {
            Ok(q) => q,
            Err(_) => {
                println!("❗ Invalid quantity");
                continue;
            }
        };

        engine.submit_order(order_type, side, price, quantity);
    }
}
