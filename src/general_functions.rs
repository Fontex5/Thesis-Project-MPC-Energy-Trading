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

    pub fn calculate_saved_energy_for_user(consumer: &mut User, time_interval:i32, appliances:&[Appliances])
    {
        let mut total_saved_energy = 0.0;
        let mut total_consumption: f32 = 0.0;
    
        for item in appliances{
            if randomly_decide_usage_of_device() {
                total_consumption += energy_consumption_of_device_in(&item,time_interval);
            }
            else {
                total_saved_energy += energy_consumption_of_device_in(&item,time_interval);
            }
        }
    
        consumer.set_saved_amount_energy(total_saved_energy);
        consumer.set_consumed_amount_energy(total_consumption);
    }

    fn randomly_decide_usage_of_device() -> bool
    {
        let decision = rand::thread_rng().gen_bool(0.5);
        decision //If true the user will use the device
    }

    fn energy_consumption_of_device_in(device: &Appliances, time_interval: i32) -> f32
     {    
        let an_hour_in_minuts = 60.0;
        let period: f32 = (time_interval as f32)/ an_hour_in_minuts;
    
        let consumed_watts = period * (device.get_average_consumption() as f32);
        consumed_watts / 1000.0
    }
}

pub mod auction_functions{
    use crate::stakeholders::user::User;
    use super::sorting::sort;
    use rand::Rng;

    pub fn randomly_set_price_for_energy_per_user(user: &mut User)
    {
        let price = rand::thread_rng().gen_range(1..=10);  
        user.set_price_for_energy(price);
    }

    pub fn calculate_price_per_energy(user:&mut User)
    {
        if user.get_saved_amount_of_energy() != 0.0 {
            user.set_price_per_energy();
        }
    }

    pub fn announce_the_winner(list_of_users:&mut Vec<User>)
    {
        sort(list_of_users);

        println!("*********************************************************");
        println!("The winner is User{}", list_of_users[0].get_user_id());
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


