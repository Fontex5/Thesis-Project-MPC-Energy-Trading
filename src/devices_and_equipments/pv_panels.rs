pub struct PVPanel {
    number_of_panels:i32,
    energy_produced_by_one_panel:f32,
}

impl PVPanel{
    pub fn equip_neighborhood_with_pv_panels(number_of_panels:i32,energy_produced_by_one_panel:f32) -> Self
    {
        Self { number_of_panels, 
               energy_produced_by_one_panel 
            }
    }

    pub fn can_pv_panel_produce_energy(hour:i32) -> bool
    {
        if hour >= 7 && hour <= 15 
        {
            return true;
        }
        else
        {
            return false;
        }
    }

    pub fn calculate_produced_energy
}