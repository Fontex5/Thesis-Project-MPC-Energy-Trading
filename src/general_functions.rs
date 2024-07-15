pub mod energy_functions{
    use crate::devices_and_equipments::home_appliances::Appliances;
    use crate::stakeholders::user::User;
    use std::io;
    use rand::Rng;

    pub fn get_time_interval() -> i32
    {
        println!("Please enter the Time Invterval (in minutes): ");
        let mut time_interval = String::new();
        io::stdin().read_line(&mut time_interval).expect("Invalid value for Time Interval"); 
        let time_interval: i32 = match time_interval.trim().parse(){
            Ok(num) => num,
            Err(_) => 30, 
        };
        time_interval
    }

    pub fn is_device_already_in_use(user_id:i32, device:&Appliances, array_of_devices_in_use:&[[bool;6]])-> bool
    {
        let device_index = get_device_index(device);
        array_of_devices_in_use[user_id as usize][device_index]
    }

    pub fn calculate_saved_energy_for_user(consumer: &mut User, hour:i32, appliances:&[Appliances],array_of_devices_in_use:&mut [[bool;6]])
    {
        let mut total_saved_energy = 0.0;
        let mut total_consumption: f32 = 0.0;
    
        for item in appliances{
            if is_device_already_in_use(consumer.get_user_id(), item, array_of_devices_in_use)
            {
                if check_if_usage_period_is_done(consumer, item, hour)
                {
                    let device_index = get_device_index(item);
                    array_of_devices_in_use[consumer.get_user_id() as usize][device_index] = false;
                }
                else {
                    continue;
                }
            }

            if randomly_decide_usage_of_device(item,hour) 
            {
                let device_index = get_device_index(item);
                array_of_devices_in_use[consumer.get_user_id() as usize][device_index] = true;

                total_consumption += energy_consumption_of_device_in(&item);
                update_finishing_time_of_device(consumer,hour,item);
            }
            else 
            {
                total_saved_energy += energy_consumption_of_device_in(&item);
            }
        }
    
        consumer.set_saved_amount_energy(total_saved_energy);
        consumer.set_consumed_amount_energy(total_consumption);
    }

    pub fn calculate_energy_consumption_regarding_pv_bss (consumer: &mut User, hour:i32, appliances:&[Appliances],
        array_of_devices_in_use:&mut [[bool;6]], produced_energy:f32) -> f32
    {
        let mut total_saved_energy = 0.0;
        let mut total_consumption: f32 = 0.0;
        let mut remainder_generated_energy = produced_energy;

        for item in appliances{
            if is_device_already_in_use(consumer.get_user_id(), item, array_of_devices_in_use)
            {
                if check_if_usage_period_is_done(consumer, item, hour)
                {
                    let device_index = get_device_index(item);
                    array_of_devices_in_use[consumer.get_user_id() as usize][device_index] = false;
                }
                else {
                    continue;
                }
            }

            if randomly_decide_usage_of_device(item,hour) 
            {
                let device_index = get_device_index(item);
                array_of_devices_in_use[consumer.get_user_id() as usize][device_index] = true;
                let device_energy_demand = energy_consumption_of_device_in(&item);

                if remainder_generated_energy > device_energy_demand
                {
                    remainder_generated_energy -= device_energy_demand;
                }
                else {
                    if consumer.get_battery_state_of_charge() > device_energy_demand
                    {
                        consumer.decharge_battery(device_energy_demand);
                    }
                    else 
                    {
                        total_consumption += device_energy_demand;
                    }
                }
                update_finishing_time_of_device(consumer,hour,item);
            }
            else 
            {
                total_saved_energy += energy_consumption_of_device_in(&item);
            }
        }
    
        consumer.set_saved_amount_energy(total_saved_energy);
        consumer.set_consumed_amount_energy(total_consumption);
        remainder_generated_energy
    }

    fn check_if_usage_period_is_done (consumer: &mut User, device:&Appliances, hour:i32) -> bool
    {
        let device_index = get_device_index(device);
        let when_device_will_be_done = consumer.get_finishing_hour_of_device(device_index);

        if when_device_will_be_done <= hour 
        {
            return true;
        }
        else {
            return false;
        }
    }

    fn get_device_index(device:&Appliances) -> usize
    {
        let device_index = match device.get_appliance_name().as_str() {
            "Heat Pump" => 0,
            "Refrigerator" => 1,
            "Electric Vehicle" => 2,
            "Washing Machine" => 3,
            "Dishwashser" => 4,
            "CookingStove" => 5,
            _ => 6,
        };

        device_index
    }

    fn randomly_decide_usage_of_device(item:&Appliances,hour:i32) -> bool
    {
        let mut generator = rand::thread_rng();
        let decision = match hour {
            0..=5 => {
                match item.get_appliance_name().as_str() {
                    "Heat Pump" => generator.gen_bool(0.7),
                    "Refrigerator" => true,
                    "Electric Vehicle" => generator.gen_bool(0.1),
                    "Washing Machine" => generator.gen_bool(0.1),
                    "Dishwashser" => generator.gen_bool(0.15),
                    "CookingStove" => generator.gen_bool(0.1),
                    _ => false,
                }
            },
            6..=12 => {
                match item.get_appliance_name().as_str() {
                    "Heat Pump" => generator.gen_bool(0.7),
                    "Refrigerator" => true,
                    "Electric Vehicle" => generator.gen_bool(0.8),
                    "Washing Machine" => generator.gen_bool(0.3),
                    "Dishwashser" => generator.gen_bool(0.6),
                    "CookingStove" => generator.gen_bool(0.8),
                    _ => false,
                }
            },
            13..=18 => {
                match item.get_appliance_name().as_str() {
                    "Heat Pump" => generator.gen_bool(0.4),
                    "Refrigerator" => true,
                    "Electric Vehicle" => generator.gen_bool(0.8),
                    "Washing Machine" => generator.gen_bool(0.3),
                    "Dishwashser" => generator.gen_bool(0.2),
                    "CookingStove" => generator.gen_bool(0.4),
                    _ => false,
                }
            },
            19..=23 => {
                match item.get_appliance_name().as_str() {
                    "Heat Pump" => generator.gen_bool(0.7),
                    "Refrigerator" => true,
                    "Electric Vehicle" => generator.gen_bool(0.4),
                    "Washing Machine" => generator.gen_bool(0.7),
                    "Dishwashser" => generator.gen_bool(0.7),
                    "CookingStove" => generator.gen_bool(0.8),
                    _ => false,
                }
            },
            _ => false,
        };

        decision //If true the user will use the device
    }

    fn energy_consumption_of_device_in(device: &Appliances) -> f32
     {    
        let an_hour_in_minuts = 60.0;
        let period: f32 = (device.get_avarage_usage_time() as f32)/ an_hour_in_minuts;
        
        let consumed_watts = period * (device.get_average_consumption() as f32);
        consumed_watts / 1000.0
    }

    fn update_finishing_time_of_device(consumer: &mut User,starting_hour:i32,device: &Appliances)
    {
        let an_hour_in_minuts = 60.0;
        let usage_period: f32 = (device.get_avarage_usage_time() as f32)/ an_hour_in_minuts;

        let finishing_hour = (usage_period + starting_hour as f32).ceil();
        let finishing_hour: i32 = (finishing_hour as i32) % 24;
        let device_index = get_device_index(device);
        consumer.set_finishing_hour_for_device_in_use(device_index, finishing_hour);
    }
}

pub mod auction_functions{
    use crate::{devices_and_equipments::battery, stakeholders::user::User};
    use rand::Rng;

    pub fn collect_offers_from_users(list_of_users:&mut Vec<User>)
    {
        for user in &mut *list_of_users
        {
            if user.get_battery_percentage() > 0 
            {
                if user.whether_sell_energy()
                {
                    let amount_of_energy_for_sale = randomly_choose_energy_amount_from_battery(user);
                    user.set_produced_amount_energy(amount_of_energy_for_sale);
                    randomly_set_price_for_energy_per_user(user);
                    user.set_price_per_energy();
                }
            }
        }
    }

    pub fn user_wants_to_sell(user: &mut User) -> bool
    {
        let mut generator = rand::thread_rng();
        let decision:bool = match user.get_battery_percentage() {
            70..=100 => generator.gen_bool(0.8),
            30..=69 => generator.gen_bool(0.5),
            5..=29 => generator.gen_bool(0.2),
            _ => false,
        };

        decision
    }

    pub fn randomly_choose_energy_amount_from_battery(user: &mut User) ->f32
    {
        let battery_percentage = user.get_battery_percentage();
        let percentage_of_battery_to_use = rand::thread_rng().gen_range(1..battery_percentage);

        battery::convert_percentage_to_energy(percentage_of_battery_to_use, user.get_battery_capacity())
    }

    pub fn randomly_set_price_for_energy_per_user(user: &mut User)
    {
        let mut generator = rand::thread_rng();

        let price = match user.get_battery_percentage() {
            70..=100 => generator.gen_range(0.0..=1.0),
            _ => generator.gen_range(0.0..=2.0),
        };

        user.set_price_for_energy(price);
    }
}

pub mod sorting
{
    use crate::stakeholders::user::User;

    pub fn sort(vector: &mut [User]) {
        let middle = vector.len() / 2;
        if vector.len() < 2 {
            return; // No need to sort vectors with one element
        }
          
        let mut sorted = vector.to_vec();
          
        sort(&mut vector[..middle]);
        sort(&mut vector[middle..]);
          
        merge(&vector[..middle], &vector[middle..], &mut sorted);
          
        vector.copy_from_slice(&sorted); // Copy the sorted result into original vector
    }
          
    fn merge(l_arr: &[User], r_arr: &[User], sorted: &mut Vec<User>)
    {
        // Current loop position in left half, right half, and sorted vector
        let (mut left, mut right, mut i) = (0, 0, 0);
          
        while left < l_arr.len() && right < r_arr.len() {
            if l_arr[left].get_price_per_energy() <= r_arr[right].get_price_per_energy() {
                sorted[i] = l_arr[left];
                i += 1;
                left += 1;
            } else {
                sorted[i] = r_arr[right];
                i += 1;
                right += 1;
              }
        }
          
        if left < l_arr.len() {
            // If there is anything left in the left half append it after sorted members
            sorted[i..].copy_from_slice(&l_arr[left..]);
        }
          
        if right < r_arr.len() {
            // If there is anything left in the right half append it after sorted members
            sorted[i..].copy_from_slice(&r_arr[right..]);
        }
    }
}


