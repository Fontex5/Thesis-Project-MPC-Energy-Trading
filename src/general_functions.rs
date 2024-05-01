pub mod general_functions
{
    pub mod energy_functions{
        use crate::home_appliances::home_appliances::Appliances;
        use crate::user::user::User;
        use std::io;
        use rand::Rng;

        pub fn get_time_interval() -> i32{
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
    
            for item in appliances{
                if randomly_decide_usage_of_device() {
                    continue;
                }
                else {
                    total_saved_energy += saved_amount_of_energy_in(&item,time_interval);
                }
            }
    
            consumer.set_saved_amount_energy(total_saved_energy);
        }

        fn randomly_decide_usage_of_device() -> bool{
            let decision = rand::thread_rng().gen_bool(0.5);
            decision //If true the user will use the device
        }

        fn saved_amount_of_energy_in(device: &Appliances, time_interval: i32) -> f32 {
        
            let an_hour_in_minuts = 60;
            let period: f32 = (time_interval as f32)/ (an_hour_in_minuts as f32);
    
            period * device.get_average_consumption()
        }
    }

    pub mod auction_functions{
        use crate::user::user::User;
        use rand::Rng;

        pub fn randomly_set_price_for_energy_per_user(user: &mut User){
            let price = rand::thread_rng().gen_range(10..=100);
            user.set_price_for_energy(price);
        }

        pub fn calculate_price_per_energy(user:&mut User)
        {
            if user.get_saved_amount_of_energy() != 0.0 {
                user.set_price_per_energy();
            }
        }
    }
}

