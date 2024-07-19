use rand::Rng;

use crate::{devices_and_equipments::{home_appliances::{Appliances, Device}, pv_panels::PVPanel}, stakeholders::household::Household};

use super::double_auction::{self, Order};

pub struct Simulator<'a> {
    number_of_houses_in_neighborhood:u8,
    list_of_households:&'a mut Vec<Household>,
    array_of_appliances:[Appliances;6]
}

impl<'a> Simulator<'a> {
    pub fn initialize_simulator(households:&'a mut Vec<Household>, n_houses:u8) -> Self
    {
        Self{
            number_of_houses_in_neighborhood:n_houses,
            list_of_households:households,
            array_of_appliances:[
                Appliances::HeatPump(Device::set_device(3000, 45)),
                Appliances::Refrigerator(Device::set_device(150, 15)),
                Appliances::ElectricVehicle(Device::set_device(100, 21)), //Average commuting distance in Denmark 22.2 kilometers, 0.346kWh for 1.6Km, 50Km average speed
                Appliances::WashingMachine(Device::set_device(1000, 60)),
                Appliances::Dishwasher(Device::set_device(1500, 120)),
                Appliances::CookingStove(Device::set_device(1500, 20))
            ]
        }
    }

    pub fn simulate_consumption_no_criterias(&mut self, hour:u8) -> (f32,f32)
    {
        let mut total_unused_amount:f32 = 0.0;
        let mut total_consumed_amount:f32 = 0.0;

        for household in &mut  *self.list_of_households{
            for device in &  self.array_of_appliances
            {
                let device_energy_demand = device.get_energy_consumption();
                if household.whether_to_use_device(&device,hour)
                {          
                    total_consumed_amount += device_energy_demand;
                }
                else
                {
                    total_unused_amount += device_energy_demand;
                }
            }
        }
        (total_unused_amount,total_consumed_amount)
    }

    # [allow(non_snake_case)]
    pub fn simulate_consumption_with_PVPanels(&mut self, hour:u8, percentage_of_houses_with_pv:u8) -> (f32,f32)
    {
        let mut total_unused_amount:f32 = 0.0;
        let mut total_consumed_amount:f32 = 0.0;
        let number_of_houses_with_pv = ((self.number_of_houses_in_neighborhood) * (100 / percentage_of_houses_with_pv)) as i32;

        let mut i = 0;
        for household in &mut  *self.list_of_households
        {
            if i < number_of_houses_with_pv
            {
                if PVPanel::can_pv_panel_produce_energy(hour)
                {
                    household.generate_energy();
                }
                i += 1;
            }
            for device in &self.array_of_appliances
            {
                let device_energy_demand = device.get_energy_consumption();
                if household.whether_to_use_device(&device, hour)
                {
                    if !household.is_demanded_energy_suppliable(device_energy_demand)
                    {
                        total_consumed_amount += device_energy_demand;
                    }
                }
                else
                {
                    total_unused_amount += device_energy_demand;
                }
            }
        }
        (total_unused_amount,total_consumed_amount)
    }

    # [allow(non_snake_case)]
    pub fn simulate_consumption_with_PVPanels_and_DA(&mut self, hour:u8, percentage_of_houses_with_pv:u8) ->(f32,Vec<Order>)
    {
        let mut buy_orders:Vec<double_auction::Order> = Vec::new();
        let mut total_unused_amount:f32 = 0.0;
        let number_of_houses_with_pv = ((self.number_of_houses_in_neighborhood) * (100 / percentage_of_houses_with_pv)) as i32;

        let mut i = 0;
        for household in &mut  *self.list_of_households
        {
            if i < number_of_houses_with_pv
            {
                if PVPanel::can_pv_panel_produce_energy(hour)
                {
                    household.generate_energy();
                }
                i += 1;
            }
            for device in &self.array_of_appliances
            {
                let device_energy_demand = device.get_energy_consumption();
                if household.whether_to_use_device(&device, hour)
                {
                    if !household.is_demanded_energy_suppliable(device_energy_demand)
                    {
                        let price:f32 = rand::thread_rng().gen_range(0.0..=2.0) * device_energy_demand;
                        buy_orders.push(Order::new_order(household.get_household_id(), price , device_energy_demand));
                    }
                }
                else
                {
                    total_unused_amount += device_energy_demand;
                }
            }
        }
        (total_unused_amount,buy_orders)
    }
}