use devices_and_equipments::{home_appliances::{Appliances,Device}, pv_panels::PVPanel};
use stakeholders::{user::User,aggregator::Aggregator,aggregator};

pub mod devices_and_equipments;
pub mod stakeholders;
pub mod general_functions;

fn main() {

    let number_of_houses_in_neighborhood = 50;
    let mut list_of_users:Vec<User> = Vec::new();
    for i in 1..=number_of_houses_in_neighborhood {
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
    
    //let mut aggregator_c = Aggregator::initialize_aggregator(100.0, 2.60);
    let pv_panels_in_neighborhood = PVPanel::equip_neighborhood_with_pv_panels(10, 300.0); 
    let potential_production_energy = pv_panels_in_neighborhood.calculate_produced_energy(60);
    let number_of_houses_with_pv_panels:i32 = ((number_of_houses_in_neighborhood as f32) * 0.2) as i32;   //The percentage of houses in the neighborhood with PV panels

    let mut total_saved_energy_without_pv:f32 = 0.0;
    let mut total_consumed_energy_without_pv:f32 = 0.0;

    for _hour in 1..=24 {
        let mut _saved:f32 = 0.0;
        let mut _consumed:f32 = 0.0;

        (_saved,_consumed) = aggregator::simulate_consumption(&mut list_of_users, &array_of_appliances);
        total_consumed_energy_without_pv += _consumed;
        total_saved_energy_without_pv += _saved;    
    }

    let mut total_saved_energy_with_pv:f32 = 0.0;
    let mut total_consumed_energy_with_pv:f32 = 0.0;
    let mut total_surplus_production:f32 = 0.0;

    for hour in 1..=24 {
        let mut _saved:f32 = 0.0;
        let mut _consumed:f32 = 0.0;
        let mut produced_energy:f32 = 0.0;
        let mut _surplus_produced: f32 = 0.0;

        if PVPanel::can_pv_panel_produce_energy(hour) {
            produced_energy = potential_production_energy;
        }

        (_saved,_consumed,_surplus_produced) = aggregator::simulate_consumption_with_pv_panels(&mut list_of_users, &array_of_appliances, produced_energy, number_of_houses_with_pv_panels);
        total_saved_energy_with_pv += _saved;
        total_consumed_energy_with_pv += _consumed;
        total_surplus_production += _surplus_produced;    
    }

    //let time_interval = energy_functions::get_time_interval();

    /*for user in &mut list_of_users{
        energy_functions::calculate_saved_energy_for_user(user, time_interval, &array_of_appliances);

        auction_functions::randomly_set_price_for_energy_per_user(user);
        auction_functions::calculate_price_per_energy(user);
    }*/
    //To remove users with no saved energy
    //list_of_users.retain(|&x| x.get_saved_amount_of_energy() != 0.0); 
    

    /*for user in &list_of_users{
        println!("----------------------------------------------");
        println!("User{} has saved {}kWh",user.get_user_id(),user.get_saved_amount_of_energy());
        println!("with total price {} DKK which is {} kWh/DKK",user.get_price_for_energy(),user.get_price_per_energy());
    }*/

    //sorting::sort(&mut list_of_users);

    //auction_functions::announce_the_winner(&mut list_of_users);
    
    //aggregator.charge_the_battery(list_of_users);

    println!("Total amount of consumed energy without PV panels: {}kWh\nTotal amount of saved energy without PV panels: {}kWh", total_consumed_energy_without_pv, total_saved_energy_without_pv);
    println!("==========================================================================================");
    println!("Total amount of consumed energy with PV panels: {}kWh\nTotal amount of saved energy with PV panels: {}kWh", total_consumed_energy_with_pv, total_saved_energy_with_pv);
    println!("And, the total remainded produced energy from the PV panels is: {}kWh",total_surplus_production);

}
