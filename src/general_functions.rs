pub mod general_functions
{
    use crate::home_appliances::home_appliances::Appliances;
    use crate::user::user::User;
    use std::io;
    use rand::Rng;

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
        let decision = rand::thread_rng().gen_range(1..=2);

        if decision == 1 {
            //The device will be used
            return true;
        }
        else{
            //The device will not be used
            return false;
        }
    }

    fn saved_amount_of_energy_in(device: &Appliances, time_interval: i32) -> f32 {
        
        let an_hour_in_minuts = 60;
        let period: f32 = (time_interval as f32)/ (an_hour_in_minuts as f32);

        period * device.get_average_consumption()
    }

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
}