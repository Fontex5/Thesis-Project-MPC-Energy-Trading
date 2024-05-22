use home_appliances::{Appliances,Device};
use crate::stakeholders::{user::User,aggregator::Aggregator};
use general_functions::*;

pub mod home_appliances;
pub mod stakeholders;
pub mod general_functions;

fn main() {

    let mut list_of_users:Vec<User> = Vec::new();
    for i in 1..=20 {
        list_of_users.push(User::initialize_user(i));
    }

    let array_of_appliances: [Appliances; 6] = [
        Appliances::HeatPump(Device::set_device(3000, 1440)),
        Appliances::Refrigerator(Device::set_device(150, 400)),
        Appliances::ElectricVehicle(Device::set_device(100, 45)), //Average commuting distance in Denmark 22.2 kilometers, 0.346kWh for 1.6Km, 50Km average speed
        Appliances::WashingMachine(Device::set_device(1000, 60)),
        Appliances::Dishwashser(Device::set_device(1500, 120)),
        Appliances::CookingStove(Device::set_device(1500, 30))
    ];
    
    let mut aggregator = Aggregator::initialize_aggregator(100.0, 2.60); 

    let time_interval = energy_functions::get_time_interval();

    for user in &mut list_of_users{
        energy_functions::calculate_saved_energy_for_user(user, time_interval, &array_of_appliances);

        auction_functions::randomly_set_price_for_energy_per_user(user);
        auction_functions::calculate_price_per_energy(user);
    }
    //To remove users with no saved energy
    //list_of_users.retain(|&x| x.get_saved_amount_of_energy() != 0.0); 
    

    for user in &list_of_users{
        println!("----------------------------------------------");
        println!("User{} has saved {}KWh",user.get_user_id(),user.get_saved_amount_of_energy());
        println!("with total price {}$ which is {} Kwh/$",user.get_price_for_energy(),user.get_price_per_energy());
    }

    sorting::sort(&mut list_of_users);

    auction_functions::announce_the_winner(&mut list_of_users);
    
    aggregator.charge_the_battery(list_of_users);

}
