pub struct Battery
{
    capacity: f32,
    percentage: i32,
}

impl Battery
{
    
    pub fn initialize_battery(capacity:f32) ->Self
    {
        Self
        {
            capacity,percentage:0
        }
    }

    pub fn charge(&mut self, received_charge:f32) -> f32
    {
        if self.percentage == 100
        {
            return received_charge;
        }
        else {
            let needed_energy = self.calculate_reuqired_energy_to_be_full();

            if received_charge > needed_energy{
                let exceeded_energy = received_charge - needed_energy;
                self.percentage = 100;
                return exceeded_energy;
            }
            else{
                self.percentage += ((received_charge * 100.0) / self.capacity) as i32;
                return 0.0;
            }
        }
    }

    fn calculate_reuqired_energy_to_be_full(&self) -> f32
    {
        let remaining_percentage = 100 - self.percentage;
        ((remaining_percentage as f32) / 100.0) * self.capacity 
    }

    pub fn is_battery_full(&self) -> bool
    {
        match self.percentage{
            100 => true,
            _ => false,
        }
    }
}