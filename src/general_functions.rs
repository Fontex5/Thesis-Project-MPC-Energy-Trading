pub mod general_functions
{
    use crate::home_appliances::home_appliances::Appliances;
    use crate::user::user::User;
    use std::io;

    pub fn calculate_saved_energy_for_user(consumer: &mut User, time_interval:i32, appliances:&[Appliances])
    {
        let mut total_saved_energy = 0.0;

        for item in appliances{
            if will_user_use_this_device(&item){
                continue;
            }
            else {
                total_saved_energy += saved_amount_of_energy_in(&item,time_interval);
            }
        }

        consumer.set_saved_amount_energy(total_saved_energy);
    }

    fn will_user_use_this_device(device: &Appliances) -> bool {
        println!("Will you use {} in the next interval? 1)Yes 2)No", device.get_appliance_name());
        
        loop{
            let mut answer = String::new();
            io::stdin()
                .read_line(&mut answer)
                .expect("Faild to read the user's answer");

            let answer: u8 = match answer.trim().parse(){
                Ok(num) => num,
                Err(_) => {println!("Please enter a number!"); continue;}, 
            };

            match answer {
                1 => return true,
                2 => return false,
                _ => {println!("Please enter a valid option!"); continue;},
            }
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