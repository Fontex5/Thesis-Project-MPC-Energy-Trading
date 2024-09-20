use crate::utilities::double_auction::{MatchedTrade, Order};

pub fn get_provider_price(hour:u8) ->f32
{
    match hour 
        {
            0 => 2.04,
            1 => 1.97,
            2 => 1.93,
            3 => 1.97,
            4 => 2.00,
            5 => 2.16,
            6 => 2.41,
            7 => 2.42,
            8 => 2.25,
            9 => 2.06,
            10 => 1.92,
            11 => 1.81,
            12 => 1.62,
            13 => 1.56,
            14 => 1.71,
            15 => 1.88,
            16 => 2.03,
            17 => 2.41,
            18 => 2.42,
            19 => 2.55,
            20 => 2.82,
            21 => 2.48,
            22 => 2.28,
            23 => 2.19,
            _ => 2.12,
        }
}

pub fn extract_consumption_and_cost(hour:u8,matched_trades:& Vec<MatchedTrade>,buy_orders:&mut Vec<Order>) -> (f32,f32)
{
    let mut cost:f32 = 0.0;
    let mut consumed_from_elec_provider:f32 = 0.0;

    for trade in matched_trades {
        println!("Seller {} sold {:.2}kWh to Buyer {} for {:.2}DKK",trade.seller_id,trade.quantity,trade.buyer_id, trade.price);
        cost += trade.price * trade.quantity;
    }

    while !buy_orders.is_empty()
    {
        cost += buy_orders[0].quantity * get_provider_price(hour);
        consumed_from_elec_provider += buy_orders[0].quantity;
        buy_orders.remove(0);
    }

    (consumed_from_elec_provider,cost)
}
