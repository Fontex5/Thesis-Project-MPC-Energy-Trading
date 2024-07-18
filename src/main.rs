use stakeholders::{household::Household,aggregator::Aggregator};
use utilities::{double_auction,simulator::Simulator};

pub mod devices_and_equipments;
pub mod stakeholders;
pub mod utilities;

const HOUR_IN_MINUTES:f32 = 60.0; 
fn main() {

    const NUMBER_OF_HOUSES_IN_NEIGHBORHOOD: u8 = 50;
    let mut list_of_households:Vec<Household> = Vec::new();
    for i in 0.. NUMBER_OF_HOUSES_IN_NEIGHBORHOOD {
        list_of_households.push(Household::initialize_household(i as i32, 40.0, 10));
    }

    let mut simulator = Simulator::initialize_simulator(&mut list_of_households,NUMBER_OF_HOUSES_IN_NEIGHBORHOOD);
    let neighborhood_aggregator = Aggregator::initialize_aggregator(2.60);
    
    let mut total_saved_energy_without_pv:f32 = 0.0;
    let mut total_consumed_energy_without_pv:f32 = 0.0;
    let mut cost_of_supplying_consumed_energy_without_pv:f32 = 0.0;

    for hour in 0..24 {
        let mut _saved:f32 = 0.0;
        let mut _consumed:f32 = 0.0;

        (_saved,_consumed) = simulator.simulate_consumption_no_criterias(hour);
        total_consumed_energy_without_pv += _consumed;
        total_saved_energy_without_pv += _saved;

        cost_of_supplying_consumed_energy_without_pv += _consumed * neighborhood_aggregator.get_provider_price(hour);    
    }

    let mut total_saved_energy_with_pv:f32 = 0.0;
    let mut total_consumed_energy_with_pv:f32 = 0.0;
    let mut total_cost_with_pv_panels:f32 = 0.0;
    

    for hour in 0..24 {
        let mut _saved:f32 = 0.0;
        let mut _consumed:f32 = 0.0;

        (_saved,_consumed) = simulator.simulate_consumption_with_PVPanels(hour,20);
        total_saved_energy_with_pv += _saved;
        total_consumed_energy_with_pv += _consumed;


        total_cost_with_pv_panels += _consumed * neighborhood_aggregator.get_provider_price(hour);
    }
    //let mut sell_orders:Vec<double_auction::Order> = Vec::new();
    //let mut buy_orders:Vec<double_auction::Order> = Vec::new();

    println!("==========================================================================================");
    println!("Total amount of consumed energy without PV panels: {}kWh\nTotal amount of saved energy without PV panels: {}kWh", total_consumed_energy_without_pv, total_saved_energy_without_pv);
    println!("Supplying the consumed energy costs {}DKK using the energy provider",cost_of_supplying_consumed_energy_without_pv);
    println!("==========================================================================================");
    println!("Total amount of consumed energy with PV panels: {}kWh\nTotal amount of saved energy with PV panels: {}kWh", total_consumed_energy_with_pv, total_saved_energy_with_pv);
    println!("With local market and electricity provider, the cost is: {}DKK",total_cost_with_pv_panels);

}
