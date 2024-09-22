#![allow(unused_variables)]

#[macro_use]
extern crate pbc_contract_codegen;
extern crate pbc_contract_common;

mod zk_compute;

use pbc_contract_common::address::Address;
use crate::zk_compute::SecretOrderStruct;
use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::context::ContractContext;
use pbc_contract_common::events::EventGroup;
use pbc_contract_common::zk::{SecretVarId, ZkInputDef, ZkState, ZkStateChange};
use read_write_state_derive::ReadWriteState;
use pbc_traits::ReadWriteState;


#[derive(ReadWriteState, Debug)]
#[repr(C)]
struct SecretVarMetadata {
    order_type: SecretVarType,
}

#[derive(ReadWriteState, Debug, PartialEq)]
#[repr(u8)]
enum SecretVarType {
    Buying = 1,
    Selling = 2,
    Matched = 3,
}

#[derive(ReadWriteState, CreateTypeSpec, Clone)]
pub struct TradeResult {
    pub buyer_id: i32,
    pub seller_id: i32,
    pub quantity: i32,
}

#[derive(ReadWriteState, CreateTypeSpec, Clone)]
pub struct ListOfTrades {
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

/// State of the contract.
#[state]
struct ContractState {

    pub auction_holder: Address,

    pub equilibrium_price: Option<i32>,

    pub matched_orders:Vec<TradeResult>,
}

#[init(zk = true)]
fn initialize(ctx: ContractContext, zk_state: ZkState<SecretVarMetadata>) -> ContractState {
    ContractState {
        auction_holder: ctx.sender,
        equilibrium_price: None,
        matched_orders: Vec::new(),
    }
}

#[zk_on_secret_input(shortname = 0x40, secret_type = "SecretOrderStruct")]
fn secret_input_buy_order(
    context: ContractContext,
    state: ContractState,
    zk_state: ZkState<SecretVarMetadata>,
) -> (
    ContractState,
    Vec<EventGroup>,
    ZkInputDef<SecretVarMetadata, SecretOrderStruct>,
) {

    let input_def =
        ZkInputDef::with_metadata(None, SecretVarMetadata{order_type:SecretVarType::Buying,});

    (state, vec![], input_def)
}

#[zk_on_secret_input(shortname = 0x45, secret_type = "SecretOrderStruct")]
fn secret_input_sell_order(
    context: ContractContext,
    state: ContractState,
    zk_state: ZkState<SecretVarMetadata>,
) -> (
    ContractState,
    Vec<EventGroup>,
    ZkInputDef<SecretVarMetadata, SecretOrderStruct>,
) {

    let input_def =
        ZkInputDef::with_metadata(None, SecretVarMetadata{order_type:SecretVarType::Selling,});

    (state, vec![], input_def)
}

#[action(shortname = 0x02, zk = true)]
fn hold_double_auction(
    context: ContractContext,
    state: ContractState,
    zk_state: ZkState<SecretVarMetadata>,
) -> (ContractState, Vec<EventGroup>, Vec<ZkStateChange>) {

    assert_eq!(state.equilibrium_price.is_none(), true, "Equilibrium price has been calculated before!");

    (
        state,
        vec![],
        vec![zk_compute::double_auction_start(
            Some(SHORTNAME_COMPUTATION_COMPLETE),
            [
                &SecretVarMetadata{order_type: SecretVarType::Matched,},
                &SecretVarMetadata{order_type: SecretVarType::Matched,}
            ],
        )],
    )
}

#[zk_on_compute_complete(shortname = 0x42)]
fn computation_complete(
    _context: ContractContext,
    state: ContractState,
    zk_state: ZkState<SecretVarMetadata>,
    output_variables: Vec<SecretVarId>,
) -> (ContractState, Vec<EventGroup>, Vec<ZkStateChange>) {
    (
        state,
        vec![],
        vec![ZkStateChange::OpenVariables {
            variables: output_variables,
        }],
    )
}

#[zk_on_variables_opened]
fn save_opened_variable(
    context: ContractContext,
    mut state: ContractState,
    zk_state: ZkState<SecretVarMetadata>,
    opened_variables: Vec<SecretVarId>,
) -> (ContractState, Vec<EventGroup>, Vec<ZkStateChange>) {
    
    assert!(
        opened_variables.len() == 2,
        "Unexpected number of output variables"
    );

    assert!(state.matched_orders.is_empty(), "Matched orders is not empty before auction!");

        let eq_price:i32 = read_variable(&zk_state,opened_variables.first());
        state.equilibrium_price = Some(eq_price); 

        let list_of_trades:ListOfTrades = read_variable(&zk_state,opened_variables.get(1));
        state.matched_orders.push(list_of_trades.trade_1);
        state.matched_orders.push(list_of_trades.trade_2);
        state.matched_orders.push(list_of_trades.trade_3);
        state.matched_orders.push(list_of_trades.trade_4);
        state.matched_orders.push(list_of_trades.trade_5);
        state.matched_orders.push(list_of_trades.trade_6);
        state.matched_orders.push(list_of_trades.trade_7);
        state.matched_orders.push(list_of_trades.trade_8);
        state.matched_orders.push(list_of_trades.trade_9);
        state.matched_orders.push(list_of_trades.trade_10);

        assert!(state.matched_orders.len() == 10, "Matched orders does not have 10 items!");

    (state, vec![], vec![])
}

fn read_variable<T: ReadWriteState>(
    zk_state: &ZkState<SecretVarMetadata>,
    variable_id: Option<&SecretVarId>,
) -> T {
    let variable_id = *variable_id.unwrap();
    let variable = zk_state.get_variable(variable_id).unwrap();
    let buffer: Vec<u8> = variable.data.clone().unwrap();
    let result = T::state_read_from(&mut buffer.as_slice());

    result
}