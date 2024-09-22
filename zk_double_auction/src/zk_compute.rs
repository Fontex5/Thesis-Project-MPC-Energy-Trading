use pbc_zk::*;
use create_type_spec_derive::CreateTypeSpec;

#[allow(unused)]
const BUYING_ORDER: u8 = 1u8;
const SELLING_ORDER: u8 = 2u8;

#[allow(unused)]
#[derive(SecretBinary, Clone, Copy, CreateTypeSpec)]
pub struct SecretOrderStruct {
    pub house_id: Sbi32,
    pub price_1: Sbi32,
    pub price_2: Sbi32,
    pub price_3: Sbi32,
    pub price_4: Sbi32,
    pub price_5: Sbi32,
    pub price_6: Sbi32,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TradeResult {
    pub buyer_id: Sbi32,
    pub seller_id: Sbi32,
    pub quantity: Sbi32,
}

#[derive(Clone, PartialEq)]
pub struct SecretTradeStruct {
    pub trade_1: TradeResult,
    pub trade_2: TradeResult,
    pub trade_3: TradeResult,
    pub trade_4: TradeResult,
    pub trade_5: TradeResult,
    pub trade_6: TradeResult,
    pub trade_7: TradeResult,
    pub trade_8: TradeResult,
    pub trade_9: TradeResult,
    pub trade_10: TradeResult, 
}

#[zk_compute(shortname = 0x62)]
pub fn double_auction() -> (Sbi32,SecretTradeStruct)
{

    let mut total_demand = [Sbi32::from(0);6];
    let mut total_supply = [Sbi32::from(0);6];

    //Extract the total demand and supply
    for var_id in secret_variable_ids()
    {
        let offer = load_sbi::<SecretOrderStruct>(var_id);

        if load_metadata::<u8>(var_id) == BUYING_ORDER
        {
            total_demand[0] = total_demand[0] + offer.price_1;
            total_demand[1] = total_demand[1] + offer.price_2;
            total_demand[2] = total_demand[2] + offer.price_3;
            total_demand[3] = total_demand[3] + offer.price_4;
            total_demand[4] = total_demand[4] + offer.price_5;
            total_demand[5] = total_demand[5] + offer.price_6;
        }
        else if load_metadata::<u8>(var_id) == SELLING_ORDER
        {
            total_supply[0] = total_supply[0] + offer.price_1;
            total_supply[1] = total_supply[1] + offer.price_2;
            total_supply[2] = total_supply[2] + offer.price_3;
            total_supply[3] = total_supply[3] + offer.price_4;
            total_supply[4] = total_supply[4] + offer.price_5;
            total_supply[5] = total_supply[5] + offer.price_6;
        }
    }

    // We should do a Binary Search, but since
    // there is no support for While, For loop is used

    let mut ep = Sbi32::from(0);
    for i in 0usize..6usize
    {
        if total_supply[5 - i] > total_demand[5 - i]
        {
            ep = Sbi32::from((5 - i) as i32);
        }
    }


    //This should be an array but since it is not supported
    //the variables are assigned individually
    let mut trades = SecretTradeStruct{
        trade_1:TradeResult{buyer_id:Sbi32::from(0),seller_id:Sbi32::from(0),quantity:Sbi32::from(0)},
        trade_2:TradeResult{buyer_id:Sbi32::from(0),seller_id:Sbi32::from(0),quantity:Sbi32::from(0)},
        trade_3:TradeResult{buyer_id:Sbi32::from(0),seller_id:Sbi32::from(0),quantity:Sbi32::from(0)},
        trade_4:TradeResult{buyer_id:Sbi32::from(0),seller_id:Sbi32::from(0),quantity:Sbi32::from(0)},
        trade_5:TradeResult{buyer_id:Sbi32::from(0),seller_id:Sbi32::from(0),quantity:Sbi32::from(0)},
        trade_6:TradeResult{buyer_id:Sbi32::from(0),seller_id:Sbi32::from(0),quantity:Sbi32::from(0)},
        trade_7:TradeResult{buyer_id:Sbi32::from(0),seller_id:Sbi32::from(0),quantity:Sbi32::from(0)},
        trade_8:TradeResult{buyer_id:Sbi32::from(0),seller_id:Sbi32::from(0),quantity:Sbi32::from(0)},
        trade_9:TradeResult{buyer_id:Sbi32::from(0),seller_id:Sbi32::from(0),quantity:Sbi32::from(0)},
        trade_10:TradeResult{buyer_id:Sbi32::from(0),seller_id:Sbi32::from(0),quantity:Sbi32::from(0)},
    };

    //Placing House_ids and their quantites in two arrays
    let mut sell_orders_quantity = [Sbi32::from(0);2];
    let mut sell_orders_ids = [Sbi32::from(0);2];
    let mut buy_orders_quantity = [Sbi32::from(0);5];
    let mut buy_orders_ids = [Sbi32::from(0);5];
    
    let mut index_b:usize = 0;
    let mut index_s:usize = 0;

    //Extract the quantity for equilibrium price
    for variable_id in secret_variable_ids()
    {
        let offer = load_sbi::<SecretOrderStruct>(variable_id);
        let mut quantity = Sbi32::from(0);

        if ep == Sbi32::from(0)
        {
            quantity = offer.price_1;
        }
        else if ep == Sbi32::from(1)
        {
            quantity = offer.price_2;
        }
        else if ep == Sbi32::from(2)
        {
            quantity = offer.price_3;
        }
        else if ep == Sbi32::from(3)
        {
            quantity = offer.price_4;
        }
        else if ep == Sbi32::from(4)
        {
            quantity = offer.price_5;
        }
        else if ep == Sbi32::from(5)
        {
            quantity = offer.price_6;
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
    for i in 0usize..2usize
    {
        for j in 0usize..5usize
        {
            let mut traded_quantity = Sbi32::from(0);
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

            let trade_resutlt:TradeResult = TradeResult{buyer_id:buy_orders_ids[j],seller_id:sell_orders_ids[i],quantity:traded_quantity};
            
            if index_t == 0
            {
                trades.trade_1 = trade_resutlt;
            }
            else if index_t == 1 
            {
                trades.trade_2 = trade_resutlt;
            }
            else if index_t == 2 
            {
                trades.trade_3 = trade_resutlt;
            }
            else if index_t == 3 
            {
                trades.trade_4 = trade_resutlt;
            }
            else if index_t == 4 
            {
                trades.trade_5 = trade_resutlt;
            }
            else if index_t == 5 
            {
                trades.trade_6 = trade_resutlt;
            }
            else if index_t == 6 
            {
                trades.trade_7 = trade_resutlt;
            }
            else if index_t == 7 
            {
                trades.trade_8 = trade_resutlt;
            }
            else if index_t == 8 
            {
                trades.trade_9 = trade_resutlt;
            }
            else if index_t == 9 
            {
                trades.trade_10 = trade_resutlt;
            }

            index_t = index_t + 1;
        }
    }

    (ep,trades)
}
