#[derive(Copy, Clone)]
pub struct User {
    id : i32,
    saved_amount_energy: f32,
    consumed_amount_energy:f32,
    produced_amount_energy:f32,
    price_for_energy: f32,
    price_per_energy: f32,
}

impl User{

    pub fn initialize_user(id:i32) ->Self
    {
        Self {  id, saved_amount_energy: 0.0,
                consumed_amount_energy: 0.0,
                produced_amount_energy: 0.0,
                price_for_energy:0.0, 
                price_per_energy:0.0 }
    }

    pub fn set_saved_amount_energy(&mut self, saved_amount: f32)
    {
        self.saved_amount_energy = saved_amount;
    }

    pub fn get_saved_amount_of_energy(&self) -> f32
    {
        self.saved_amount_energy
    }

    pub fn set_consumed_amount_energy(&mut self, consumed_amount: f32)
    {
        self.consumed_amount_energy = consumed_amount;
    }

    pub fn get_consumed_amount_of_energy(&self) -> f32
    {
        self.consumed_amount_energy
    }

    pub fn set_produced_amount_energy(&mut self, produced_amount:f32)
    {
        self.produced_amount_energy = produced_amount;
    }

    pub fn get_produced_amount_of_energy(&self) -> f32
    {
        self.produced_amount_energy
    }

    pub fn get_user_id(&self) -> i32
    {
        self.id
    }

    pub fn set_price_for_energy(&mut self, price:f32)
    {
        self.price_for_energy = price;
    }

    pub fn get_price_for_energy(&self) ->f32
    {
        self.price_for_energy
    }

    pub fn set_price_per_energy(&mut self)
    {
        self.price_per_energy =  self.price_for_energy / self.produced_amount_energy;
    }

    pub fn get_price_per_energy(&self) -> f32
    {
        self.price_per_energy
    }
}
