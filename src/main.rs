use stakeholders::{household::Household,aggregator};
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
    
    let mut total_consumed_energy_without_pv:f32 = 0.0;
    let mut total_cost_without_pv_panels:f32 = 0.0;


    let mut total_consumed_energy_with_pv:f32 = 0.0;
    let mut total_cost_with_pv_panels:f32 = 0.0;  


    let mut total_consumed_energy_with_da:f32 = 0.0;
    let mut total_cost_with_da:f32 = 0.0;
    for hour in 0..24 {
        let mut sell_orders:Vec<double_auction::Order> = Vec::new();
        let mut buy_orders:Vec<double_auction::Order> = Vec::new();

        let consumed_amounts = simulator.simulate_consumption(hour,20, &mut buy_orders, &mut sell_orders);

        total_consumed_energy_without_pv += consumed_amounts.0;
        total_cost_without_pv_panels += consumed_amounts.0 * aggregator::get_provider_price(hour);

        total_consumed_energy_with_pv += consumed_amounts.1;
        total_cost_with_pv_panels += consumed_amounts.1 * aggregator::get_provider_price(hour);
        

        let matched_trades = double_auction::double_auction(&mut buy_orders, &mut sell_orders);
        simulator.decharge_houses_which_sold_energy(&matched_trades);

        let da_consumption_and_cost = aggregator::extract_consumption_and_cost(hour, &matched_trades, &mut buy_orders); 
        total_consumed_energy_with_da += da_consumption_and_cost.0; 
        total_cost_with_da += da_consumption_and_cost.1;
    }

    println!("==========================================================================================");
    println!("Total amount of consumed energy without PV panels: {}kWh", total_consumed_energy_without_pv);
    println!("Supplying the consumed energy costs {}DKK using the energy provider",total_cost_without_pv_panels);
    println!("==========================================================================================");
    println!("Total amount of consumed energy with PV panels: {}kWh", total_consumed_energy_with_pv);
    println!("Supplying the consumed energy costs {}DKK using the energy provider",total_cost_with_pv_panels);
    println!("==========================================================================================");
    println!("Total amount of consumed energy with Double Auction: {}kWh", total_consumed_energy_with_da);
    println!("With Double Auction and electricity provider, the cost is: {}DKK",total_cost_with_da);

}
