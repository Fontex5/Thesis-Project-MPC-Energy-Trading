use std::io;
use crate::devices_and_equipments::home_appliances::Appliances;
use crate::general_functions::{auction_functions, energy_functions};

use super::user::User;

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

    pub fn get_provider_price(&self,hour:i32) ->f32
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

    pub fn calculate_cost_for_hour(&self, list_of_users:&mut Vec<User>, hour:i32,mut demanded_energy:f32) -> f32
    {
        auction_functions::collect_offers_from_users(list_of_users);

        let mut cost:f32 = 0.0;

        while demanded_energy > 0.0
        {
            for user in &mut *list_of_users
            {
                if user.get_produced_amount_of_energy() > 0.0
                {
                    if user.get_price_per_energy() < self.get_provider_price(hour)
                    {
                        demanded_energy -= user.get_produced_amount_of_energy();
                        cost += user.get_price_for_energy();
                        user.decharge_battery(user.get_produced_amount_of_energy());
                        user.set_produced_amount_energy(0.0);
                    }
                }
            }
            cost += demanded_energy * self.get_provider_price(hour);
            demanded_energy = 0.0;
        }

        cost
    }
}

pub fn get_price_from_electricity_provider() -> f32
{
    println!("Please enter the price offered by the\n electricity provider for 1Kwh (as a floating number): ");
    let mut price = String::new();
    io::stdin().read_line(&mut price).expect("Invalid value for the price"); 
    let price: f32 = match price.trim().parse(){
        Ok(num) => num,
        Err(_) => 20.0, 
    };
    price
}

pub fn set_battery_capacity() -> f32
{
    println!("Please enter the Battery Capacity (in Kwh): ");
    let mut battery_cap = String::new();
    io::stdin().read_line(&mut battery_cap).expect("Invalid value for the battery capacity"); 
    let battery_cap: f32 = match battery_cap.trim().parse(){
        Ok(num) => num,
        Err(_) => 100.0, 
    };
    battery_cap
}

pub fn simulate_consumption(list_of_users:&mut Vec<User>, array_of_appliances:&[Appliances], array_of_devices_in_use:&mut[[bool;6]],hour:i32) -> (f32 ,f32)
{
    let mut total_saved_amount:f32 = 0.0;
    let mut total_consumed_amount:f32 = 0.0;

    for user in list_of_users{
        energy_functions::calculate_saved_energy_for_user(user, hour, &array_of_appliances, array_of_devices_in_use);
        total_saved_amount += user.get_saved_amount_of_energy();
        total_consumed_amount += user.get_consumed_amount_of_energy();
    }

    (total_saved_amount,total_consumed_amount)
}

pub fn simulate_consumption_with_pv_panels(list_of_users:&mut Vec<User>, array_of_appliances:&[Appliances], array_of_devices_in_use:&mut[[bool;6]],hour:i32, produced_energy:f32, number_of_houses_with_pv_panels:i32) -> (f32 ,f32)
{
    let mut total_saved_amount:f32 = 0.0;
    let mut total_consumed_amount:f32 = 0.0;
    let mut total_surplus_production:f32 = 0.0;
    let mut number_of_considered_house_with_pv = 0;

    for user in list_of_users{
        
        if (number_of_considered_house_with_pv < number_of_houses_with_pv_panels) && (produced_energy != 0.0) 
        {
            let remainder_energy = energy_functions::calculate_energy_consumption_regarding_pv_bss(user, hour, &array_of_appliances, array_of_devices_in_use,produced_energy);
            if remainder_energy > 0.0
            {
                if remainder_energy < user.get_required_energy_to_full_battery() 
                {
                    user.charge_battery(remainder_energy);
                }
                else 
                {
                    //User cannot save it and DSO will not pay for it 
                    total_surplus_production += remainder_energy;    
                }
            }
            number_of_considered_house_with_pv += 1;
        }
        else 
        {
            energy_functions::calculate_saved_energy_for_user(user, hour, &array_of_appliances, array_of_devices_in_use);
        } 
        
        total_saved_amount += user.get_saved_amount_of_energy();
        total_consumed_amount += user.get_consumed_amount_of_energy();
    }
    total_consumed_amount -= total_surplus_production;

    (total_saved_amount,total_consumed_amount)
}

