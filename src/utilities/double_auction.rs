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

    if buy_orders.is_empty() || sell_orders.is_empty()
    {
        return matched_trades;
    }

    buy_orders.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
    sell_orders.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

    // Equilibrium price must be found based on the breakeven index
    let mut _equilibrium_price:f32 = 0.0;
    let mut _index_k:usize = 0;

    if buy_orders.len() > sell_orders.len()
    {
        _index_k = sell_orders.len() - 1;
    }
    else 
    {
        _index_k = buy_orders.len() - 1;  
    }
    while buy_orders[_index_k].price < sell_orders[_index_k].price
    {
        _index_k -= 1;
    }

    _equilibrium_price = (buy_orders[_index_k].price + sell_orders[_index_k].price) / 2.0; 

    // Do the trading
    while !buy_orders.is_empty() && !sell_orders.is_empty() {
        let buy_order = buy_orders[0];
        let sell_order = sell_orders[0];

        if buy_order.price >= sell_order.price {
            let trade_quantity = buy_order.quantity.min(sell_order.quantity);
            matched_trades.push(MatchedTrade::new_trade(buy_order.household_id, sell_order.household_id, _equilibrium_price, trade_quantity));

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