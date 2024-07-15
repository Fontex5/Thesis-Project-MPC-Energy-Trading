#[derive(Copy, Clone)]
pub struct Battery
{
    capacity: f32,
    percentage: u8,
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

    pub fn charge(&mut self, received_charge:f32)
    {
        self.percentage += ((received_charge * 100.0 as f32) / self.capacity).ceil() as u8
    }

    pub fn calculate_reuqired_energy_to_be_full(&self) -> f32
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

    pub fn get_percentage(&self) -> u8
    {
        self.percentage
    }

    pub fn get_capacity(&self) -> f32
    {
        self.capacity
    }

    pub fn decharge(&mut self, used_energy:f32)
    {
        let used_percentage = convert_energy_to_percentage(used_energy, self.capacity);
        self.percentage = self.percentage - used_percentage;
    }

    pub fn state_of_charge(&self) -> f32
    {
        (self.percentage as f32 * self.capacity)/100.0 as f32
    }
}

pub fn convert_percentage_to_energy(percentage:u8, capacity:f32) ->f32
{
    (percentage as f32 / 100.0) * capacity
}

pub fn convert_energy_to_percentage(energy:f32,capacity:f32) -> u8
{
    ((energy * 100.0 as f32) / capacity).ceil() as u8
}