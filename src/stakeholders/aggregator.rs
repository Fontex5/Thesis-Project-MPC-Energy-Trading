use std::io;
use crate::devices_and_equipments::{home_appliances::Appliances,pv_panels};
use crate::general_functions::energy_functions;

use super::user::User;

pub struct Aggregator
{
    battery_capacity: f32,
    battery_percentage: i32,
    price_received_by_elec_provider: f32
}

impl Aggregator
{
    pub fn initialize_aggregator(battery_capacity:f32, price_received_by_elec_provider:f32) ->Self
    {
        Aggregator { battery_capacity, battery_percentage: 0 , price_received_by_elec_provider }
    }

    pub fn set_battery_capacity(&mut self, capacity:f32)
    {
        self.battery_capacity = capacity;
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

    pub fn supply_demand_with_pv(&self, list_of_users:Vec<User>,mut demanded_energy:f32) -> f32
    {
        let mut i = 0;
        let mut total_price = 0.0;

        while demanded_energy != 0.0
        {
            if list_of_users[i].get_price_per_energy() < self.price_received_by_elec_provider {
                demanded_energy -= list_of_users[i].get_produced_amount_of_energy();
                //list_of_users[i].set_produced_amount_energy(0.0);
                total_price += list_of_users[i].get_price_for_energy();
                i += 1; 
            }
            else {
                total_price += demanded_energy * self.price_received_by_elec_provider;
                demanded_energy = 0.0;
            }
        }

        total_price
    }

    fn charge_battery(&mut self, received_charge:f32)
    {
        if self.battery_percentage == 100
        {
            println!("Battery is full. Cannot charge anymore!");
        }
        else {
            let needed_energy = self.calculate_reuqired_energy_for_full_battery();

            if received_charge > needed_energy{
                let exceeded_energy = received_charge - needed_energy;
                println!("Battery is full now and {}kWh is exceeded",exceeded_energy);
                self.battery_percentage = 100;
            }
            else{
                self.battery_percentage += ((received_charge * 100.0) / self.battery_capacity) as i32;
            }
        }
    }

    fn calculate_reuqired_energy_for_full_battery(&self) -> f32
    {
        let remaining_percentage = 100 - self.battery_percentage;
        ((remaining_percentage as f32) / 100.0) * self.battery_capacity 
    }

    pub fn is_battery_full(&self) -> bool
    {
        match self.battery_percentage{
            100 => true,
            _ => false,
        }
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

pub fn simulate_consumption_with_pv_panels(list_of_users:&mut Vec<User>, array_of_appliances:&[Appliances], array_of_devices_in_use:&mut[[bool;6]],hour:i32, produced_energy:f32, number_of_houses_with_pv_panels:i32) -> (f32 ,f32,f32)
{
    let mut total_saved_amount:f32 = 0.0;
    let mut total_consumed_amount:f32 = 0.0;
    let mut total_surplus_production:f32 = 0.0;
    let mut number_of_considered_house_with_pv = 0;

    for user in list_of_users{
        energy_functions::calculate_saved_energy_for_user(user, hour, &array_of_appliances, array_of_devices_in_use);
        
        if number_of_considered_house_with_pv < number_of_houses_with_pv_panels 
        {
            pv_panels::deduct_produced_energy_from_consumption(user, produced_energy);
            number_of_considered_house_with_pv += 1;
        } 
        
        total_saved_amount += user.get_saved_amount_of_energy();
        total_consumed_amount += user.get_consumed_amount_of_energy();
        total_surplus_production += user.get_produced_amount_of_energy();
    }

    (total_saved_amount,total_consumed_amount, total_surplus_production)
}