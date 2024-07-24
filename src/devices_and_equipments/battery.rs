#[derive(Copy, Clone)]
pub struct Battery
{
    capacity: f32,
    percentage: u8,
    state_of_charge: f32,
}

impl Battery
{
    
    pub fn initialize_battery(capacity:f32) ->Self
    {
        Self
        {
            capacity,
            percentage:0,
            state_of_charge:0.0
        }
    }

    pub fn charge(&mut self, received_charge:f32)
    {
        self.state_of_charge += received_charge;
        self.update_percentage();
    }

    fn update_percentage(&mut self)
    {
        self.percentage = ((self.state_of_charge * 100.0 as f32) / self.capacity) as u8;
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
        self.state_of_charge -= used_energy;
        self.update_percentage();
    }

    pub fn get_state_of_charge(&self) -> f32
    {
        self.state_of_charge
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