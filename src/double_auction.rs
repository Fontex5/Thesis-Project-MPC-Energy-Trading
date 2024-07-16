#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Order {
    price: f32,
    quantity: f32
}

pub fn double_auction(mut buy_orders: Vec<Order>, mut sell_orders: Vec<Order>) -> Vec<(f32, f32)> {
    let mut matched_trades = Vec::new();
    buy_orders.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
    println!("{:#?}",buy_orders);
    println!("---------------------------------------------------");
    sell_orders.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
    println!("{:#?}",sell_orders);


    while !buy_orders.is_empty() && !sell_orders.is_empty() {
        let buy_order = buy_orders[0];
        let sell_order = sell_orders[0];

        if buy_order.price >= sell_order.price {
            let trade_price = (buy_order.price + sell_order.price) / 2.0;
            let trade_quantity = buy_order.quantity.min(sell_order.quantity);
            matched_trades.push((trade_price, trade_quantity));

            buy_orders[0].quantity -= trade_quantity;
            sell_orders[0].quantity -= trade_quantity;

            if buy_orders[0].quantity == 0.0 {
                buy_orders.remove(0);
            }
            if sell_orders[0].quantity == 0.0 {
                sell_orders.remove(0);
            }
        } else {
            break;
        }
    }

    matched_trades
}