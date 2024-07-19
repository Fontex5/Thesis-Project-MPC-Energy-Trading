use crate::utilities::double_auction::{self, double_auction};
use super::household::Household;

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

    pub fn calculate_cost_for_hour(&self, list_of_households:&mut Vec<Household>, hour:u8,buy_orders:&mut Vec<double_auction::Order>,sell_orders:&mut Vec<double_auction::Order>) -> f32
    {
        double_auction::collect_offers_from_households(list_of_households, sell_orders);

        let mut cost:f32 = 0.0;

        let matched_trades = double_auction(buy_orders, sell_orders);
        for trade in matched_trades {
            let seller_id = trade.seller_id as usize;

            println!("Seller{} Trade with Buyer{} at price ${:.2}, quantity {}",seller_id,trade.buyer_id, trade.price, trade.quantity);
            list_of_households[seller_id].set_produced_amount_energy(0.0);
            list_of_households[seller_id].decharge_battery(trade.quantity);

            cost += trade.price * trade.quantity;
        }

        while !buy_orders.is_empty()
        {
            cost += buy_orders[0].quantity * self.get_provider_price(hour);
            buy_orders.remove(0);
        }

        cost
    }
}
