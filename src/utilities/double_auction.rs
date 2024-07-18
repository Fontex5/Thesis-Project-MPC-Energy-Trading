use crate::{devices_and_equipments::battery, stakeholders::household::Household};
use rand::Rng;

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

pub fn collect_offers_from_households(list_of_households:&mut Vec<Household>,sell_orders:&mut Vec<crate::double_auction::Order>)
{
    for household in &mut *list_of_households
    {
        if household.get_battery_percentage() > 0 
        {
            if household.whether_to_sell_energy()
            {
                let amount_of_energy_for_sale = randomly_choose_energy_amount_from_battery(household);
                household.set_produced_amount_energy(amount_of_energy_for_sale);
                randomly_set_price_for_energy_per_household(household);
                household.set_price_per_energy();
                sell_orders.push(Order::new_order(household.get_household_id(), household.get_price_for_energy(), amount_of_energy_for_sale));
            }
        }
    }
}

pub fn household_wants_to_sell(household: &mut Household) -> bool
{
    let mut generator = rand::thread_rng();
    let decision:bool = match household.get_battery_percentage() {
        70..=100 => generator.gen_bool(0.8),
        30..=69 => generator.gen_bool(0.5),
        5..=29 => generator.gen_bool(0.2),
        _ => false,
    };

    decision
}

pub fn randomly_choose_energy_amount_from_battery(household: &mut Household) ->f32
{
    let battery_percentage = household.get_battery_percentage();
    let percentage_of_battery_to_use = rand::thread_rng().gen_range(1..battery_percentage);

    battery::convert_percentage_to_energy(percentage_of_battery_to_use, household.get_battery_capacity())
}

pub fn randomly_set_price_for_energy_per_household(household: &mut Household)
{
    let mut generator = rand::thread_rng();

    let price = match household.get_battery_percentage() {
        70..=100 => generator.gen_range(0.0..=1.0),
        _ => generator.gen_range(0.0..=2.0),
    };

    household.set_price_for_energy(price);
}