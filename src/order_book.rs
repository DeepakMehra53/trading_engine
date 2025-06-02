use std::collections::{BTreeMap, VecDeque};
use crate::order::{Order, OrderSide};
use ordered_float::OrderedFloat;

pub struct OrderBook {
    pub buy_orders: BTreeMap<f64, VecDeque<Order>>,  // Highest price first
    pub sell_orders: BTreeMap<f64, VecDeque<Order>>, // Lowest price first
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            buy_orders: BTreeMap::new(),
            sell_orders: BTreeMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        let book = match order.side {
            OrderSide::Buy => &mut self.buy_orders,
            OrderSide::Sell => &mut self.sell_orders,
        };

        let price = order.price.unwrap(); // Only limit orders go in the book
        book.entry(price).or_insert_with(VecDeque::new).push_back(order);
    }

    pub fn get_best_price(&self, side: OrderSide) -> Option<f64> {
        match side {
            OrderSide::Buy => self.buy_orders.keys().rev().next().copied(),
            OrderSide::Sell => self.sell_orders.keys().next().copied(),
        }
    }

    pub fn pop_best_order(&mut self, side: OrderSide) -> Option<Order> {
        let book = match side {
            OrderSide::Buy => &mut self.buy_orders,
            OrderSide::Sell => &mut self.sell_orders,
        };

        let key = match side {
            OrderSide::Buy => book.keys().rev().next().copied(),
            OrderSide::Sell => book.keys().next().copied(),
        };

        if let Some(price) = key {
            if let Some(queue) = book.get_mut(&price) {
                let order = queue.pop_front();
                if queue.is_empty() {
                    book.remove(&price);
                }
                return order;
            }
        }
        None
    }
}
