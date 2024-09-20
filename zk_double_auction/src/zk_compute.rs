use pbc_zk::*;

#[allow(unused)]
const BUYING_ORDER: u8 = 1u8;

#[allow(unused)]

#[zk_compute(shortname = 0x61)]
pub fn find_equilibrium_price() -> Sbi32
{  
    let mut total_demand = [Sbi32::from(0);10];
    let mut total_supply = [Sbi32::from(0);10];

    for var_id in secret_variable_ids()
    {
        let offer = load_sbi::<[Sbi32;10]>(var_id);

        if load_metadata::<u8>(var_id) == BUYING_ORDER
        {
            for i in 0..10
            {
                total_demand[i] = total_demand[i] + offer[i];
            }
        }
        else
        {
            for i in 0..10
            {
                total_supply[i] = total_supply[i] + offer[i];
            }
        }
    }

    //Binary Search
    let mut left: usize = 0;

    let result = total_supply[left];
    result
}