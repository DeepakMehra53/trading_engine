use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderType {
    Market,
    Limit,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: u64,
    pub order_type: OrderType,
    pub side: OrderSide,
    pub price: Option<f64>, // Only for Limit orders
    pub quantity: f64,
    pub timestamp: u128,
}

impl Order {
    pub fn new(id: u64, order_type: OrderType, side: OrderSide, price: Option<f64>, quantity: f64) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        Self {
            id,
            order_type,
            side,
            price,
            quantity,
            timestamp,
        }
    }
}
