use pbc_zk::*;
use create_type_spec_derive::CreateTypeSpec;

#[allow(unused)]
const BUYING_ORDER: u8 = 1u8;
const SELLING_ORDER: u8 = 2u8;

#[allow(unused)]
#[derive(SecretBinary, Clone, Copy, CreateTypeSpec)]
pub struct SecretOrderStruct {
    pub house_id: Sbi16,
    pub quantity_per_price:[Sbi16;6],
}

#[derive(Clone, PartialEq)]
pub struct SecretTradeStruct {
    pub buyer_ids: [Sbi16;135],
    pub seller_ids: [Sbi16;135],
    pub quantities: [Sbi16;135],
}

#[zk_compute(shortname = 0x62)]
pub fn double_auction() -> (Sbi16,SecretTradeStruct)
{

    let mut total_demand = [Sbi16::from(0);6];
    let mut total_supply = [Sbi16::from(0);6];

    //Extract the total demand and supply
    for var_id in secret_variable_ids()
    {
        let offer = load_sbi::<SecretOrderStruct>(var_id);

        if load_metadata::<u8>(var_id) == BUYING_ORDER
        {
            for i in 0usize..6usize
            {
                total_demand[i] = total_demand[i] + offer.quantity_per_price[i];
            }
        }
        else if load_metadata::<u8>(var_id) == SELLING_ORDER
        {
            for i in 0usize..6usize
            {
                total_supply[i] = total_supply[i] + offer.quantity_per_price[i];
            }
        }
    }

    // We should do a Binary Search, but since
    // there is no support for While, For loop is used

    let mut ep = Sbi16::from(0);
    for i in 0usize..6usize
    {
        if total_supply[5 - i] > total_demand[5 - i]
        {
            ep = Sbi16::from((5 - i) as i16);
        }
    }

    let mut res = SecretTradeStruct{
        buyer_ids:[Sbi16::from(0);135],
        seller_ids:[Sbi16::from(0);135],
        quantities:[Sbi16::from(0);135],
    };

    //Placing House_ids and their quantities in two arrays
    let mut sell_orders_quantity = [Sbi16::from(0);3];
    let mut sell_orders_ids = [Sbi16::from(0);3];
    let mut buy_orders_quantity = [Sbi16::from(0);45];
    let mut buy_orders_ids = [Sbi16::from(0);45];
    
    let mut index_b:usize = 0;
    let mut index_s:usize = 0;

    //Extract the quantity for equilibrium price
    for variable_id in secret_variable_ids()
    {
        let offer = load_sbi::<SecretOrderStruct>(variable_id);
        let mut quantity = Sbi16::from(0);

        for i in 0usize..6usize
        {
            if ep == Sbi16::from(i as i16)
            {
                quantity = offer.quantity_per_price[i];
            }
        }

        if load_metadata::<u8>(variable_id) == BUYING_ORDER
        {
            buy_orders_quantity[index_b] = quantity;
            buy_orders_ids[index_b] = offer.house_id;
            index_b = index_b + 1;
        }
        else if load_metadata::<u8>(variable_id) == SELLING_ORDER
        {
            sell_orders_quantity[index_s] = quantity;
            sell_orders_ids[index_s] = offer.house_id;
            index_s = index_s + 1;
        }
    }
    
    let mut index_t:usize = 0;

    //Trading between all elements
    for i in 0usize..3usize
    {
        for j in 0usize..45usize
        {
            let mut traded_quantity = Sbi16::from(0);
            if sell_orders_quantity[i] > buy_orders_quantity[j]
            {
                traded_quantity = buy_orders_quantity[j];
            }
            else
            {
                traded_quantity = sell_orders_quantity[i];
            }

            sell_orders_quantity[i] = sell_orders_quantity[i] - traded_quantity;
            buy_orders_quantity[j] = buy_orders_quantity[j] - traded_quantity;

            res.buyer_ids[index_t] = buy_orders_ids[j];
            res.seller_ids[index_t] = sell_orders_ids[i];
            res.quantities[index_t] = traded_quantity;

            index_t = index_t + 1;
        }
    }

    (ep,res)
}
