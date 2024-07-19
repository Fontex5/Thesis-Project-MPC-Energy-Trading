#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Order {
   pub price: f32,
   pub quantity: f32,
   pub household_id: i32
}

impl Order {
    pub fn new_order(household_id:i32, price:f32, quantity:f32) ->Self
    {
        Self { price, quantity, household_id }
    }
}

pub struct MatchedTrade
{
    pub buyer_id: i32,
    pub seller_id: i32,
    pub price: f32,
    pub quantity:f32
}

impl MatchedTrade {
    pub fn new_trade(buyer_id:i32, seller_id:i32, price:f32, quantity:f32) -> Self
    {
        Self {buyer_id,seller_id,price,quantity}
    }
}

pub fn double_auction(buy_orders: &mut Vec<Order>, sell_orders:&mut Vec<Order>) -> Vec<MatchedTrade> {
    let mut matched_trades:Vec<MatchedTrade> = Vec::new();
    buy_orders.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
    sell_orders.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());


    while !buy_orders.is_empty() && !sell_orders.is_empty() {
        let buy_order = buy_orders[0];
        let sell_order = sell_orders[0];

        if buy_order.price >= sell_order.price {
            let trade_price = (buy_order.price + sell_order.price) / 2.0;
            let trade_quantity = buy_order.quantity.min(sell_order.quantity);
            matched_trades.push(MatchedTrade::new_trade(buy_order.household_id, sell_order.household_id, trade_price, trade_quantity));

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