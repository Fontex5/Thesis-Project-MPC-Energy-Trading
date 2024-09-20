#![allow(unused_variables)]

#[macro_use]
extern crate pbc_contract_codegen;
extern crate pbc_contract_common;

mod zk_compute;

use pbc_contract_common::address::Address;
use pbc_contract_common::context::ContractContext;
use pbc_contract_common::events::EventGroup;
use pbc_contract_common::zk::{SecretVarId, ZkInputDef, ZkState, ZkStateChange};
use read_write_state_derive::ReadWriteState;
use pbc_traits::ReadWriteState;
use pbc_zk::Sbi32;


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

/// State of the contract.
#[state]
struct ContractState {

    pub auction_holder: Address,

    pub equilibrium_price: i32,

    pub deadline_utc_millis: i64,
}

#[init(zk = true)]
fn initialize(ctx: ContractContext, zk_state: ZkState<SecretVarMetadata>, deadline_utc_millis: i64) -> ContractState {
    ContractState {
        auction_holder: ctx.sender,
        equilibrium_price: 0,
        deadline_utc_millis,
    }
}

#[zk_on_secret_input(shortname = 0x40)]
fn secret_input_buy_order(
    context: ContractContext,
    state: ContractState,
    zk_state: ZkState<SecretVarMetadata>,
) -> (
    ContractState,
    Vec<EventGroup>,
    ZkInputDef<SecretVarMetadata, [Sbi32;10]>,
) {

    let input_def =
        ZkInputDef::with_metadata(None, SecretVarMetadata{order_type:SecretVarType::Buying,});

    (state, vec![], input_def)
}

#[zk_on_secret_input(shortname = 0x45)]
fn secret_input_sell_order(
    context: ContractContext,
    state: ContractState,
    zk_state: ZkState<SecretVarMetadata>,
) -> (
    ContractState,
    Vec<EventGroup>,
    ZkInputDef<SecretVarMetadata, [Sbi32;10]>,
) {

    let input_def =
        ZkInputDef::with_metadata(None, SecretVarMetadata{order_type:SecretVarType::Selling,});

    (state, vec![], input_def)
}

#[action(shortname = 0x01, zk = true)]
fn find_equilibrium_price(
    context: ContractContext,
    state: ContractState,
    zk_state: ZkState<SecretVarMetadata>,
) -> (ContractState, Vec<EventGroup>, Vec<ZkStateChange>) {
    (
        state,
        vec![],
        vec![zk_compute::find_equilibrium_price_start(
            Some(SHORTNAME_COMPUTATION_COMPLETE),
            &SecretVarMetadata{order_type: SecretVarType::Matched,},
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
        opened_variables.len() == 1,
        "Unexpected number of output variables"
    );

    let eq_price:i32 = read_variable(&zk_state,opened_variables.first());
    state.equilibrium_price = eq_price; 

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