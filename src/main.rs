use stakeholders::{household::Household,aggregator};
use utilities::{double_auction,simulator::Simulator};

pub mod devices_and_equipments;
pub mod stakeholders;
pub mod utilities;

const HOUR_IN_MINUTES:f32 = 60.0;
const FEED_IN_TARIFF:f32 = 0.58; 
fn main() {

    const NUMBER_OF_HOUSES_IN_NEIGHBORHOOD: u8 = 50;
    let mut list_of_households:Vec<Household> = Vec::new();
    for i in 0.. NUMBER_OF_HOUSES_IN_NEIGHBORHOOD {
        list_of_households.push(Household::initialize_household(i as i32, 40.0, 20));
    }
    let mut simulator = Simulator::initialize_simulator(&mut list_of_households,NUMBER_OF_HOUSES_IN_NEIGHBORHOOD);

    csv_handler::create_csv_files();
    
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

        if let Err(err) = csv_handler::write_record_to_csv("consumption_with_da.csv",(hour,da_consumption_and_cost.0)){
            println!("{}", err);
        }
    }

    println!("==========================================================================================");
    println!("Total amount of consumed energy without PV panels: {:.2}kWh", total_consumed_energy_without_pv);
    println!("Supplying the consumed energy costs {:.2}DKK using the energy provider",total_cost_without_pv_panels);
    println!("==========================================================================================");
    println!("Total amount of consumed energy with PV panels: {:.2}kWh", total_consumed_energy_with_pv);
    println!("Supplying the consumed energy costs {:.2}DKK using the energy provider",total_cost_with_pv_panels);
    println!("==========================================================================================");
    println!("Total amount of consumed energy with Double Auction: {:.2}kWh", total_consumed_energy_with_da);
    println!("With Double Auction and electricity provider, the cost is: {:.2}DKK",total_cost_with_da);

}

pub mod csv_handler
{
    use csv::Writer;
    use std::fs::OpenOptions;
    use std::error::Error;
    use std::process;

    pub fn initialize_csv_output(file_name:&str,data_type:&Vec<&str>) -> Result<(), Box<dyn Error>>
    {
        let mut wtr = Writer::from_path(file_name)?;
        //let mut wtr = csv::Writer::from_writer(io::stdout());
        // Since we're writing records manually, we must explicitly write our
        // header record. A header record is written the same way that other
        // records are written.
        wtr.write_record(data_type)?;

        // A CSV writer maintains an internal buffer, so it's important
        // to flush the buffer when you're done.
        wtr.flush()?;
        Ok(())
    }

    pub fn write_record_to_csv(file_name:&str,record:(u8,f32)) -> Result<(), Box<dyn Error>>
    {
        let file = OpenOptions::new().append(true).open(file_name)?;
        let mut wtr = Writer::from_writer(file);
        //let mut wtr = csv::Writer::from_writer(io::stdout());
        // Since we're writing records manually, we must explicitly write our
        // header record. A header record is written the same way that other
        // records are written.
        wtr.serialize(record)?;
    
        // A CSV writer maintains an internal buffer, so it's important
        // to flush the buffer when you're done.
        wtr.flush()?;
        Ok(())
    }

    pub fn create_csv_files()
    {
        if let Err(err) = initialize_csv_output("consumption_without_pv.csv",&vec!["hour","consumption"]) {
            println!("{}", err);
            process::exit(1);
        }
        if let Err(err) = initialize_csv_output("household_without_pv.csv",&vec!["hour","consumption"]) {
            println!("{}", err);
            process::exit(1);
        }
        if let Err(err) = initialize_csv_output("household_with_pv.csv",&vec!["hour","consumption"]) {
            println!("{}", err);
            process::exit(1);
        }
        if let Err(err) = initialize_csv_output("consumption_with_pv.csv",&vec!["hour","consumption"]) {
            println!("{}", err);
            process::exit(1);
        }
        if let Err(err) = initialize_csv_output("consumption_with_da.csv",&vec!["hour","consumption"]) {
            println!("{}", err);
            process::exit(1);
        }
    }
}