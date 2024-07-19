use crate::HOUR_IN_MINUTES;

#[derive(Copy, Clone)]
pub struct Device
{
    average_consumption_in_watts: i32, 
    average_time_usage_minutes: i32,  
}

impl Device
{
    pub fn set_device(consumption_in_watts:i32, avg_time:i32 ) -> Self
    {
        Self{
            average_consumption_in_watts: consumption_in_watts,
            average_time_usage_minutes: avg_time
        }
    }

    pub fn get_average_consumption(&self)->i32{
        self.average_consumption_in_watts
    }

    pub fn get_avarage_usage_time(&self) -> i32
    {
        self.average_time_usage_minutes
    }
}

pub enum Appliances
{
    HeatPump(Device),
    Refrigerator(Device),
    ElectricVehicle(Device),
    WashingMachine(Device),
    Dishwasher(Device),
    CookingStove(Device)
}

impl Appliances
{
    pub fn get_average_consumption(&self) ->i32
    {
        match self {
            Self::HeatPump(device) => device.get_average_consumption(),
            Self::Refrigerator(device) => device.get_average_consumption(),
            Self::ElectricVehicle(device) => device.get_average_consumption(),
            Self::WashingMachine(device) => device.get_average_consumption(),
            Self::Dishwasher(device) => device.get_average_consumption(),
            Self::CookingStove(device) => device.get_average_consumption(),
        }
    }

    pub fn get_appliance_name(&self) -> String
    {
        match self{
            Self::HeatPump(_) => String::from("Heat Pump"),
            Self::Refrigerator(_) => String::from("Refrigerator"),
            Self::ElectricVehicle(_) => String::from("Electric Vehicle"),
            Self::WashingMachine(_) => String::from("Washing Machine"),
            Self::Dishwasher(_) => String::from("Dishwashser"),
            Self::CookingStove(_) => String::from("CookingStove"),
        }
    }

    pub fn get_avarage_usage_time(&self) -> i32
    {
        match self {
            Self::HeatPump(device) => device.get_avarage_usage_time(),
            Self::Refrigerator(device) => device.get_avarage_usage_time(),
            Self::ElectricVehicle(device) => device.get_avarage_usage_time(),
            Self::WashingMachine(device) => device.get_avarage_usage_time(),
            Self::Dishwasher(device) => device.get_avarage_usage_time(),
            Self::CookingStove(device) => device.get_avarage_usage_time(),
        }
    }

    pub fn get_position_index(&self) -> usize
    {
        let device_index = match self {
            Self::HeatPump(_) => 0,
            Self::Refrigerator(_) => 1,
            Self::ElectricVehicle(_) => 2,
            Self::WashingMachine(_) => 3,
            Self::Dishwasher(_) => 4,
            Self::CookingStove(_) => 5,
        };

        device_index
    }

    pub fn get_energy_consumption(&self) -> f32
    {    
       let period: f32 = (self.get_avarage_usage_time() as f32)/ HOUR_IN_MINUTES;      
       let consumed_watts = period * (self.get_average_consumption() as f32);
       consumed_watts / 1000.0
   }
}

