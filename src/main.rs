use home_appliances::home_appliances::{Appliances,Device};
use user::user::User;
use general_functions::general_functions::*;

pub mod home_appliances;
pub mod user;
pub mod general_functions;

fn main() {

    let mut list_of_users:Vec<User> = Vec::new();
    for i in 1..=20 {
        list_of_users.push(User::initialize_user(i));
    }

    let array_of_appliances: [Appliances; 5] = [
        Appliances::HeatPump(Device::set_device(30.0, 1440)),
        Appliances::Refrigerator(Device::set_device(0.5, 1440)),
        Appliances::ElectricVehicle(Device::set_device(4.4, 30)),
        Appliances::WashingMachine(Device::set_device(1.0, 60)),
        Appliances::Dishwashser(Device::set_device(1.5, 120))
    ];

    let time_interval = general_functions::general_functions::get_time_interval();

    for user in &mut list_of_users{
        calculate_saved_energy_for_user(user, time_interval, &array_of_appliances);
    }
    

    for user in &list_of_users{
        println!("User{} has saved {}KWh",user.get_user_id(),user.get_saved_amount_of_energy());
    }
}
