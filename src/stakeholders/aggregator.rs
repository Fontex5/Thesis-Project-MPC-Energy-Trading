use crate::utilities::double_auction::{MatchedTrade, Order};

pub struct Aggregator
{
    price_received_by_elec_provider: f32
}

impl Aggregator
{
    pub fn initialize_aggregator(price_received_by_elec_provider:f32) ->Self
    {
        Aggregator { price_received_by_elec_provider }
    }

    pub fn set_provider_price(&mut self, price:f32)
    {
        self.price_received_by_elec_provider = price;
    }

    pub fn get_provider_price(&self,hour:u8) ->f32
    {
        let price_change:f32 = match hour 
        {
            6..=12 => 1.2,
            13..=18 => 1.4,
            19..=23 => 0.8,
            0..=5 => 0.4,
            _ => 1.0,
        };

        self.price_received_by_elec_provider * price_change
    }

    pub fn extract_consumption_and_cost(&self,hour:u8,matched_trades:& Vec<MatchedTrade>,buy_orders:&mut Vec<Order>) -> (f32,f32)
    {
        let mut cost:f32 = 0.0;
        let mut consumed_from_elec_provider:f32 = 0.0;

        for trade in matched_trades {
            let seller_id = trade.seller_id as usize;

            println!("Seller{} Trade with Buyer{} at price ${:.2}, quantity {}",seller_id,trade.buyer_id, trade.price, trade.quantity);
            cost += trade.price * trade.quantity;
        }

        while !buy_orders.is_empty()
        {
            cost += buy_orders[0].quantity * self.get_provider_price(hour);
            consumed_from_elec_provider += buy_orders[0].quantity;
            buy_orders.remove(0);
        }

    (consumed_from_elec_provider,cost)
    }
}
