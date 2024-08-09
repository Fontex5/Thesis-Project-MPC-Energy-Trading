use rand::Rng;

use crate::{devices_and_equipments::{home_appliances::{Appliances, Device}, pv_panels::PVPanel}, stakeholders::{aggregator, household::Household}};

use super::double_auction::{MatchedTrade, Order};

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
                Appliances::TV(Device::set_device(120, 45)),
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
    pub fn simulate_consumption_with_PVPanels(&mut self, hour:u8, percen_houses_with_pv:u8) -> (f32,f32)
    {
        let mut total_unused_amount:f32 = 0.0;
        let mut total_consumed_amount:f32 = 0.0;
        let number_of_houses_with_pv = ((self.number_of_houses_in_neighborhood as f32) * (percen_houses_with_pv as f32 / 100.0)).ceil() as i32;

        let mut i = 0;
        for household in &mut  *self.list_of_households
        {
            if i < number_of_houses_with_pv
            {
                if PVPanel::can_pv_panel_produce_energy(hour)
                {
                    if !household.is_battery_full()
                    {
                        household.generate_energy();
                    }
                    else 
                    {
                        //household cannot save the generated energy and cause imbalance
                        //in the grid, therefore, the generated energy is take for free
                        total_consumed_amount -= household.get_generated_energy();
                    }
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
    pub fn simulate_consumption_with_PVPanels_and_DA(&mut self, hour:u8, percen_houses_with_pv:u8, buy_orders:&mut Vec<Order>, sell_orders:&mut Vec<Order>) -> f32
    {
        let mut total_unused_amount:f32 = 0.0;
        let number_of_houses_with_pv = ((self.number_of_houses_in_neighborhood as f32) * (percen_houses_with_pv as f32 / 100.0)).ceil() as i32;

        let mut i = 0;
        for household in &mut  *self.list_of_households
        {
            if i < number_of_houses_with_pv
            {
                if PVPanel::can_pv_panel_produce_energy(hour)
                {
                    if !household.is_battery_full()
                    {
                        household.generate_energy();
                    }
                    else 
                    {
                        //household cannot save the generated energy and cause imbalance
                        //in the grid, therefore, the generated energy is take for free
                        sell_orders.push(Order::new_order(household.get_household_id(), 0.0, household.get_generated_energy()));
                    }
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
                        //Set the price for 1 kWh of energy
                        let maximum_price = aggregator::get_provider_price(hour);
                        let price:f32 = rand::thread_rng().gen_range(0.1..maximum_price) * device_energy_demand;
                        buy_orders.push(Order::new_order(household.get_household_id(), price , device_energy_demand));
                    }
                }
                else
                {
                    total_unused_amount += device_energy_demand;
                }
            }

            //Check if household would like to sell energy
            if household.whether_to_sell_energy()
            {
                sell_orders.push(household.offer_sell_order(hour));
            }
        }
        total_unused_amount
    }

    pub fn simulate_consumption(&mut self, hour:u8, percen_houses_with_pv:u8, buy_orders:&mut Vec<Order>, sell_orders:&mut Vec<Order>)->(f32,f32)
    {
        let mut consumption_without_pv:f32 = 0.0;
        let mut consumption_with_pv:f32 = 0.0;
        let number_of_houses_with_pv = ((self.number_of_houses_in_neighborhood as f32) * (percen_houses_with_pv as f32 / 100.0)).ceil() as i32;
        let maximum_price = aggregator::get_provider_price(hour);

        let mut i = 0;
        for household in &mut  *self.list_of_households
        {
            if i < number_of_houses_with_pv
            {
                if PVPanel::can_pv_panel_produce_energy(hour)
                {
                    if !household.is_battery_full()
                    {
                        household.generate_energy();
                    }
                    else 
                    {
                        //household cannot save the generated energy and cause imbalance
                        //in the grid, therefore, the generated energy is take for free
                        sell_orders.push(Order::new_order(household.get_household_id(), 0.0, household.get_generated_energy()));
                        consumption_with_pv -= household.get_generated_energy();
                    }
                }
                i += 1;
            }
            for device in &self.array_of_appliances
            {
                let device_energy_demand = device.get_energy_consumption();
                if household.whether_to_use_device(&device, hour)
                {
                    consumption_without_pv += device_energy_demand;
                    if !household.is_demanded_energy_suppliable(device_energy_demand)
                    {
                        consumption_with_pv += device_energy_demand;
 
                        let price:f32 = rand::thread_rng().gen_range(0.1..maximum_price);
                        buy_orders.push(Order::new_order(household.get_household_id(), price , device_energy_demand));
                    }
                }
            }

            //Check if household would like to sell energy
            if household.whether_to_sell_energy()
            {
                sell_orders.push(household.offer_sell_order(hour));
            }
        }
        (consumption_without_pv,consumption_with_pv)
    }

    pub fn decharge_houses_which_sold_energy(&mut self,matched_trades:&Vec<MatchedTrade>)
    {
        //This function is required since simulator has access to the list of households
        //And, the list couldn't be accessed from Aggregator since it would be access from
        //two different places
        for trade in matched_trades
        {
            self.list_of_households[trade.seller_id as usize].decharge_battery(trade.quantity);
        }
    }
}