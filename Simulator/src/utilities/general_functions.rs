pub mod energy_functions{
    use crate::devices_and_equipments::home_appliances::Appliances;
    use rand::Rng;

    pub fn randomly_decide_usage_of_device(item:&Appliances,hour:u8) -> bool
    {
        let mut generator = rand::thread_rng();
        let decision = match hour {
            0..=2 => {
                match item.get_appliance_name().as_str() {
                    "Heat Pump" => generator.gen_bool(0.6),
                    "Refrigerator" => true,
                    "TV" => generator.gen_bool(0.3),
                    "Washing Machine" => generator.gen_bool(0.1),
                    "Dishwashser" => generator.gen_bool(0.1),
                    "CookingStove" => generator.gen_bool(0.1),
                    _ => false,
                }
            },
            3..=5 => {
                match item.get_appliance_name().as_str() {
                    "Heat Pump" => generator.gen_bool(0.6),
                    "Refrigerator" => true,
                    "TV" => generator.gen_bool(0.1),
                    "Washing Machine" => generator.gen_bool(0.1),
                    "Dishwashser" => generator.gen_bool(0.1),
                    "CookingStove" => generator.gen_bool(0.1),
                    _ => false,
                }
            },
            6..=8 => {
                match item.get_appliance_name().as_str() {
                    "Heat Pump" => generator.gen_bool(0.4),
                    "Refrigerator" => true,
                    "TV" => generator.gen_bool(0.3),
                    "Washing Machine" => generator.gen_bool(0.1),
                    "Dishwashser" => generator.gen_bool(0.2),
                    "CookingStove" => generator.gen_bool(0.4),
                    _ => false,
                }
            },
            9..=11 => {
                match item.get_appliance_name().as_str() {
                    "Heat Pump" => generator.gen_bool(0.3),
                    "Refrigerator" => true,
                    "TV" => generator.gen_bool(0.6),
                    "Washing Machine" => generator.gen_bool(0.2),
                    "Dishwashser" => generator.gen_bool(0.4),
                    "CookingStove" => generator.gen_bool(0.8),
                    _ => false,
                }
            },
            12..=14 => {
                match item.get_appliance_name().as_str() {
                    "Heat Pump" => generator.gen_bool(0.2),
                    "Refrigerator" => true,
                    "TV" => generator.gen_bool(0.4),
                    "Washing Machine" => generator.gen_bool(0.4),
                    "Dishwashser" => generator.gen_bool(0.3),
                    "CookingStove" => generator.gen_bool(0.5),
                    _ => false,
                }
            },
            15..=17 => {
                match item.get_appliance_name().as_str() {
                    "Heat Pump" => generator.gen_bool(0.3),
                    "Refrigerator" => true,
                    "TV" => generator.gen_bool(0.3),
                    "Washing Machine" => generator.gen_bool(0.3),
                    "Dishwashser" => generator.gen_bool(0.1),
                    "CookingStove" => generator.gen_bool(0.3),
                    _ => false,
                }
            },
            18..=20 => {
                match item.get_appliance_name().as_str() {
                    "Heat Pump" => generator.gen_bool(0.4),
                    "Refrigerator" => true,
                    "TV" => generator.gen_bool(0.85),
                    "Washing Machine" => generator.gen_bool(0.2),
                    "Dishwashser" => generator.gen_bool(0.2),
                    "CookingStove" => generator.gen_bool(0.7),
                    _ => false,
                }
            },
            21..=23 => {
                match item.get_appliance_name().as_str() {
                    "Heat Pump" => generator.gen_bool(0.5),
                    "Refrigerator" => true,
                    "TV" => generator.gen_bool(0.7),
                    "Washing Machine" => generator.gen_bool(0.5),
                    "Dishwashser" => generator.gen_bool(0.6),
                    "CookingStove" => generator.gen_bool(0.3),
                    _ => false,
                }
            },
            _ => false,
        };

        decision //If true the household will use the device
    }
}


