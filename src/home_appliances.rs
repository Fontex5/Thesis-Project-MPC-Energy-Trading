pub struct Device
{
    average_consumption_in_watts: i32,  //In KWh
    average_time_usage_minutes: i32,  //In Minutes
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
}

pub enum Appliances
{
    HeatPump(Device),
    Refrigerator(Device),
    ElectricVehicle(Device),
    WashingMachine(Device),
    Dishwashser(Device),
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
            Self::Dishwashser(device) => device.get_average_consumption(),
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
            Self::Dishwashser(_) => String::from("Dishwashser"),
            Self::CookingStove(_) => String::from("CookingStove"),
        }
    }
}
