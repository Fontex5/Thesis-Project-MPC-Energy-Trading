use crate::utilities::double_auction::{MatchedTrade, Order};

pub fn get_provider_price(hour:u8) ->f32
{
    let base_price:f32 = 2.60;
    let price_change:f32 = match hour 
        {
            6..=12 => 1.2,
            13..=18 => 1.4,
            19..=23 => 0.8,
            0..=5 => 0.4,
            _ => 1.0,
        };
        base_price * price_change
}

pub fn extract_consumption_and_cost(hour:u8,matched_trades:& Vec<MatchedTrade>,buy_orders:&mut Vec<Order>) -> (f32,f32)
{
    let mut cost:f32 = 0.0;
    let mut consumed_from_elec_provider:f32 = 0.0;

    for trade in matched_trades {
        println!("Seller {} sold {:.2}kWh to Buyer {} for {:.2}DKK",trade.seller_id,trade.quantity,trade.buyer_id, trade.price);
        cost += trade.price; //The prices are for the whole traded energy
    }

    while !buy_orders.is_empty()
    {
        cost += buy_orders[0].quantity * get_provider_price(hour);
        consumed_from_elec_provider += buy_orders[0].quantity;
        buy_orders.remove(0);
    }

    (consumed_from_elec_provider,cost)
}
