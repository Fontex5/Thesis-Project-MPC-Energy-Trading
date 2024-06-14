use crate::stakeholders::user::User;

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
        match hour {
            7..=15 => return true,
            _ => return false,
        }
    }

    pub fn calculate_produced_energy(self, time_interval:i32) -> f32
    {
        let an_hour_in_minuts = 60.0;
        let period: f32 = (time_interval as f32)/ an_hour_in_minuts;
    
        let produced_watts = period * (self.number_of_panels as f32) * self.energy_produced_by_one_panel;
        produced_watts / 1000.0
    }
}

pub fn deduct_produced_energy_from_consumption(user:& mut User, produced_energy:f32) -> f32
{
    let remainder_energy = produced_energy - user.get_consumed_amount_of_energy();

    if remainder_energy >= 0.0 
    {
        //The house's energy usage is covered by the PV panel
        user.set_consumed_amount_energy(0.0);
    }
    else {
        let consumed_energy = remainder_energy * -1.0;
        user.set_consumed_amount_energy(consumed_energy);
    }

    remainder_energy
}