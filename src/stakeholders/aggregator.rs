use std::io;
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

    pub fn charge_the_battery(&mut self, list_of_users:Vec<User>)
    {
        let mut i = 0;
        let mut total_price = 0;

        while self.battery_percentage != 100
        {
            if list_of_users[i].get_price_per_energy() < self.price_received_by_elec_provider {
                self.charge_battery(list_of_users[i].get_saved_amount_of_energy());
                total_price += list_of_users[i].get_price_for_energy();
                i += 1; 
            }
            else {
                let required_energy = self.calculate_reuqired_energy_for_full_battery();
                self.charge_battery(required_energy);
                total_price += (required_energy * self.price_received_by_elec_provider) as i32;
            }
        }

        println!("Full charge of battery cost {}$", total_price);
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
                println!("Battery is full now and {}Kwh is exceeded",exceeded_energy);
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