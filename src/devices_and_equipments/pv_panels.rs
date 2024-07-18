use crate::{stakeholders::household::Household, HOUR_IN_MINUTES};
#[derive(Copy, Clone)]
pub struct PVPanel {
    number_of_panels:u16,
    energy_produced_by_one_panel:f32,
}

impl PVPanel{
    pub fn equip_household_with_pv_panels(number_of_panels:u16,energy_produced_by_one_panel:f32) -> Self
    {
        Self { number_of_panels, 
               energy_produced_by_one_panel 
            }
    }

    pub fn can_pv_panel_produce_energy(hour:u8) -> bool
    {
        match hour {
            8..=14 => return true,
            _ => return false,
        }
    }

    pub fn calculate_produced_energy(self, time_interval:i32) -> f32
    {
        let period: f32 = (time_interval as f32)/ HOUR_IN_MINUTES;
        let produced_watts = period * (self.number_of_panels as f32) * self.energy_produced_by_one_panel;
        produced_watts / 1000.0
    }
}

pub fn deduct_produced_energy_from_consumption(household:& mut Household, produced_energy:f32) -> f32
{
    let remainder_energy = produced_energy - household.get_consumed_amount_of_energy();

    if remainder_energy >= 0.0 
    {
        //The house's energy usage is covered by the PV panel
        household.set_consumed_amount_energy(0.0);
    }
    else {
        let consumed_energy = remainder_energy * -1.0;
        household.set_consumed_amount_energy(consumed_energy);
    }

    remainder_energy
}