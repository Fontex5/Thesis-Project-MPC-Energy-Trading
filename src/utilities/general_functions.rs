pub mod energy_functions{
    use crate::devices_and_equipments::home_appliances::Appliances;
    use crate::HOUR_IN_MINUTES;
    use rand::Rng;

    pub fn get_device_index(device:&Appliances) -> usize
    {
        let device_index = match device.get_appliance_name().as_str() {
            "Heat Pump" => 0,
            "Refrigerator" => 1,
            "Electric Vehicle" => 2,
            "Washing Machine" => 3,
            "Dishwashser" => 4,
            "CookingStove" => 5,
            _ => 6,
        };

        device_index
    }

    pub fn randomly_decide_usage_of_device(item:&Appliances,hour:u8) -> bool
    {
        let mut generator = rand::thread_rng();
        let decision = match hour {
            0..=5 => {
                match item.get_appliance_name().as_str() {
                    "Heat Pump" => generator.gen_bool(0.7),
                    "Refrigerator" => true,
                    "Electric Vehicle" => generator.gen_bool(0.1),
                    "Washing Machine" => generator.gen_bool(0.1),
                    "Dishwashser" => generator.gen_bool(0.15),
                    "CookingStove" => generator.gen_bool(0.1),
                    _ => false,
                }
            },
            6..=12 => {
                match item.get_appliance_name().as_str() {
                    "Heat Pump" => generator.gen_bool(0.7),
                    "Refrigerator" => true,
                    "Electric Vehicle" => generator.gen_bool(0.8),
                    "Washing Machine" => generator.gen_bool(0.3),
                    "Dishwashser" => generator.gen_bool(0.6),
                    "CookingStove" => generator.gen_bool(0.8),
                    _ => false,
                }
            },
            13..=18 => {
                match item.get_appliance_name().as_str() {
                    "Heat Pump" => generator.gen_bool(0.4),
                    "Refrigerator" => true,
                    "Electric Vehicle" => generator.gen_bool(0.8),
                    "Washing Machine" => generator.gen_bool(0.3),
                    "Dishwashser" => generator.gen_bool(0.2),
                    "CookingStove" => generator.gen_bool(0.4),
                    _ => false,
                }
            },
            19..=23 => {
                match item.get_appliance_name().as_str() {
                    "Heat Pump" => generator.gen_bool(0.7),
                    "Refrigerator" => true,
                    "Electric Vehicle" => generator.gen_bool(0.4),
                    "Washing Machine" => generator.gen_bool(0.7),
                    "Dishwashser" => generator.gen_bool(0.7),
                    "CookingStove" => generator.gen_bool(0.8),
                    _ => false,
                }
            },
            _ => false,
        };

        decision //If true the household will use the device
    }

    pub fn device_energy_consumption(device: &Appliances) -> f32
     {    
        let period: f32 = (device.get_avarage_usage_time() as f32)/ HOUR_IN_MINUTES;      
        let consumed_watts = period * (device.get_average_consumption() as f32);
        consumed_watts / 1000.0
    }
}


