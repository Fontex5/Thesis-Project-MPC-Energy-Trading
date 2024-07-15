use rand::Rng;

use crate::devices_and_equipments::battery::Battery;

#[derive(Copy, Clone)]
pub struct User {
    id : i32,
    saved_amount_energy: f32,
    consumed_amount_energy:f32,
    produced_amount_energy:f32,
    price_for_energy: f32,
    price_per_energy: f32,
    hours_devices_are_done:[i32;6],
    battery:Battery,
}

impl User{

    pub fn initialize_user(id:i32,battery_capacity:f32) ->Self
    {
        Self {  id, saved_amount_energy: 0.0,
                consumed_amount_energy: 0.0,
                produced_amount_energy: 0.0,
                price_for_energy:0.0, 
                price_per_energy:0.0,
                hours_devices_are_done:[0;6],
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

    pub fn get_user_id(&self) -> i32
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

    pub fn get_finishing_hour_of_device(&self,device_index:usize) -> i32
    {
        self.hours_devices_are_done[device_index]
    }

    pub fn set_finishing_hour_for_device_in_use(&mut self, device_index:usize, finishing_hour:i32)
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
    
    pub fn whether_sell_energy(&self) -> bool
    {
        let mut generator = rand::thread_rng();
        match self.battery.get_percentage() {
            70..=100 => generator.gen_bool(0.8),
            30..=69 => generator.gen_bool(0.5),
            5..=29 => generator.gen_bool(0.2),
            _ => false,
        }
    }
}
