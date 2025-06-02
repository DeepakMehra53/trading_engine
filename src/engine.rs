use crate::order::{Order, OrderSide, OrderType};
use crate::order_book::OrderBook;

pub struct MatchingEngine {
    pub order_book: OrderBook,
    pub next_order_id: u64,
}

impl MatchingEngine {
    pub fn new() -> Self {
        Self {
            order_book: OrderBook::new(),
            next_order_id: 1,
        }
    }

    pub fn submit_order(&mut self, order_type: OrderType, side: OrderSide, price: Option<f64>, quantity: f64) {
        let id = self.next_order_id;
        self.next_order_id += 1;

        let order = Order::new(id, order_type.clone(), side.clone(), price, quantity);

        match order_type {
            OrderType::Market => self.execute_market_order(order),
            OrderType::Limit => self.execute_limit_order(order),
        }
    }

    fn execute_market_order(&mut self, mut order: Order) {
        let opposite = if order.side == OrderSide::Buy { OrderSide::Sell } else { OrderSide::Buy };

        while order.quantity > 0.0 {
            if let Some(best_price) = self.order_book.get_best_price(opposite) {
                if let Some(mut best_match) = self.order_book.pop_best_order(opposite) {
                    let traded_qty = order.quantity.min(best_match.quantity);
                    println!(
                        "Trade: {} {:?} @ {} for {}",
                        order.side, OrderType::Market, best_price, traded_qty
                    );

                    order.quantity -= traded_qty;
                    best_match.quantity -= traded_qty;

                    if best_match.quantity > 0.0 {
                        self.order_book.add_order(best_match);
                    }
                }
            } else {
                println!("Market order not fully filled");
                break;
            }
        }
    }

    fn execute_limit_order(&mut self, mut order: Order) {
        let opposite = if order.side == OrderSide::Buy { OrderSide::Sell } else { OrderSide::Buy };

        while order.quantity > 0.0 {
            if let Some(best_price) = self.order_book.get_best_price(opposite) {
                let limit_price = order.price.unwrap();

                let match_possible = match order.side {
                    OrderSide::Buy => best_price <= limit_price,
                    OrderSide::Sell => best_price >= limit_price,
                };

                if !match_possible {
                    break;
                }

                if let Some(mut best_match) = self.order_book.pop_best_order(opposite) {
                    let traded_qty = order.quantity.min(best_match.quantity);
                    println!(
                        "Trade: {} {:?} @ {} for {}",
                        order.side, OrderType::Limit, best_price, traded_qty
                    );

                    order.quantity -= traded_qty;
                    best_match.quantity -= traded_qty;

                    if best_match.quantity > 0.0 {
                        self.order_book.add_order(best_match);
                    }
                }
            } else {
                break;
            }
        }

        if order.quantity > 0.0 {
            self.order_book.add_order(order);
        }
    }
}
