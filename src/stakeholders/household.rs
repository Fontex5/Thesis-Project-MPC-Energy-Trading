use rand::Rng;
use crate::devices_and_equipments::pv_panels::PVPanel;
use crate::utilities::general_functions::energy_functions;
use crate::devices_and_equipments::{battery::Battery, home_appliances::Appliances};
use crate::HOUR_IN_MINUTES;

#[derive(Copy, Clone)]
pub struct Household {
    id : i32,
    saved_amount_energy: f32,
    consumed_amount_energy:f32,
    produced_amount_energy:f32,
    price_for_energy: f32,
    price_per_energy: f32,
    array_of_devices_in_use:[bool;6],
    hours_devices_are_done:[u8;6],
    pv_panels:PVPanel,
    battery:Battery,
}

impl Household{

    pub fn initialize_household(id:i32,battery_capacity:f32, number_of_equiped_pv_panels:u16) ->Self
    {
        Self {  id, saved_amount_energy: 0.0,
                consumed_amount_energy: 0.0,
                produced_amount_energy: 0.0,
                price_for_energy:0.0, 
                price_per_energy:0.0,
                array_of_devices_in_use:[false;6],
                hours_devices_are_done:[0;6],
                pv_panels:PVPanel::equip_household_with_pv_panels(number_of_equiped_pv_panels,300.0),
                battery: Battery::initialize_battery(battery_capacity) }
    }

    pub fn set_saved_amount_energy(&mut self, saved_amount: f32)
    {
        self.saved_amount_energy = saved_amount;
    }

    pub fn get_saved_amount_of_energy(&self) -> f32
    {
        self.saved_amount_energy
    }

    pub fn set_consumed_amount_energy(&mut self, consumed_amount: f32)
    {
        self.consumed_amount_energy = consumed_amount;
    }

    pub fn get_consumed_amount_of_energy(&self) -> f32
    {
        self.consumed_amount_energy
    }

    pub fn set_produced_amount_energy(&mut self, produced_amount:f32)
    {
        self.produced_amount_energy = produced_amount;
    }

    pub fn get_produced_amount_of_energy(&self) -> f32
    {
        self.produced_amount_energy
    }

    pub fn get_household_id(&self) -> i32
    {
        self.id
    }

    pub fn set_price_for_energy(&mut self, price:f32)
    {
        self.price_for_energy = price;
    }

    pub fn get_price_for_energy(&self) ->f32
    {
        self.price_for_energy
    }

    pub fn set_price_per_energy(&mut self)
    {
        self.price_per_energy =  self.price_for_energy / self.produced_amount_energy;
    }

    pub fn get_price_per_energy(&self) -> f32
    {
        self.price_per_energy
    }

    pub fn get_finishing_hour_of_device(&self,device_index:usize) -> u8
    {
        self.hours_devices_are_done[device_index]
    }

    pub fn set_finishing_hour_for_device_in_use(&mut self, device_index:usize, finishing_hour:u8)
    {
        self.hours_devices_are_done[device_index] = finishing_hour;
    }

    pub fn is_battery_full(&self) -> bool
    {
        self.battery.is_battery_full()
    }

    pub fn charge_battery(&mut self,received_energy:f32)
    {
        self.battery.charge(received_energy);
    }

    pub fn get_battery_percentage(&self) -> u8
    {
        self.battery.get_percentage()
    }

    pub fn get_battery_capacity(&self) -> f32
    {
        self.battery.get_capacity()
    }

    pub fn get_battery_state_of_charge(&self) -> f32
    {
        self.battery.state_of_charge()
    }
    pub fn decharge_battery(&mut self, energy:f32)
    {
        self.battery.decharge(energy);
    }

    pub fn get_required_energy_to_full_battery(&self) -> f32
    {
        self.battery.calculate_reuqired_energy_to_be_full()
    }
    
    pub fn whether_to_sell_energy(&self) -> bool
    {
        let mut generator = rand::thread_rng();
        match self.battery.get_percentage() {
            70..=100 => generator.gen_bool(0.8),
            30..=69 => generator.gen_bool(0.5),
            5..=29 => generator.gen_bool(0.2),
            _ => false,
        }
    }

    pub fn generate_energy(&mut self)
    {
        let generated_energy = self.pv_panels.calculate_produced_energy(60);
        self.charge_battery(generated_energy); 
    }

    pub fn is_demanded_energy_suppliable(&mut self,demanded_energy:f32) ->bool
    {
        if self.battery.get_percentage() == 0
        {
            return false;
        }
        else 
        {
            if self.battery.state_of_charge() >= demanded_energy
            {
                self.decharge_battery(demanded_energy);
                return true;
            }
            else 
            {
                return false;
            }
        }
    }

    pub fn whether_to_use_device(&mut self, device:&Appliances,hour:u8) -> bool
    {
        if self.is_device_already_in_use(device)
        {
            if self.check_if_usage_period_is_done(device,hour)
            {
                //If the usage period is over, then it can be used again
                self.array_of_devices_in_use[device.get_position_index()] = false;
            }
            else 
            {
                return false;
            }
        }

        if energy_functions::randomly_decide_usage_of_device(device,hour) 
        {
            self.array_of_devices_in_use[device.get_position_index()] = true;
            self.update_finishing_time_of_device(hour,device);

            return true;
        }
        else 
        {
            return false;
        }
    }

    fn is_device_already_in_use(&self,device:&Appliances) ->bool
    {
        self.array_of_devices_in_use[device.get_position_index()]
    }

    fn check_if_usage_period_is_done(&self, device:&Appliances, hour:u8)-> bool
    {
        let when_device_will_be_done = self.hours_devices_are_done[device.get_position_index()];

        if when_device_will_be_done <= hour 
        {
            return true;
        }
        else {
            return false;
        }
    }

    fn update_finishing_time_of_device(&mut self,starting_hour:u8,device: &Appliances)
    {
        let usage_period: f32 = (device.get_avarage_usage_time() as f32)/ HOUR_IN_MINUTES;
        let finishing_hour = (usage_period + starting_hour as f32).ceil();
        let finishing_hour: u8 = (finishing_hour as u8) % 24;
        
        self.hours_devices_are_done[device.get_position_index()] = finishing_hour;
    }
}
