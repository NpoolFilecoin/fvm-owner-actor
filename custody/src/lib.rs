#[macro_use]
extern crate abort;

use fvm_ipld_encoding::RawBytes;
use fvm_ipld_encoding::Cbor;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_shared::address::Address;

use state::{State, Beneficiary};
use params::deserialize;

#[derive(Serialize_tuple, Deserialize_tuple, Clone)]
pub struct CustodyMinerParams {
    pub miner_id: Address,
    pub beneficiaries: Vec<Beneficiary>,
}
impl Cbor for CustodyMinerParams {}

pub fn custody_miner(params: u32) -> Option<RawBytes> {
    let params = match deserialize::<CustodyMinerParams>(params) {
        Ok(params) => params,
        Err(err) => abort!(USR_SERIALIZATION, "{:?}", err),
    };

    let state = State::load();
    match state.miners.get(&params.miner_id) {
        Some(_) => abort!(USR_ILLEGAL_STATE, "exist miner"),
        None => {},
    }

    // TODO: get miner power
    // TODO: get miner collateral
    // TODO: get miner available balance
    // TODO: get miner vesting

    state.save();

    abort!(USR_UNHANDLED_MESSAGE, "not implemented")
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone)]
pub struct ChangeWorkerAddressParams {
    pub miner_id: Address,
    pub new_worker: Address,
}
impl Cbor for ChangeWorkerAddressParams {}

pub fn change_worker_address(params: u32) -> Option<RawBytes> {
    let _params = match deserialize::<ChangeWorkerAddressParams>(params) {
        Ok(params) => params,
        Err(err) => abort!(USR_SERIALIZATION, "{:?}", err),
    };

    abort!(USR_UNHANDLED_MESSAGE, "not implemented")
}

pub fn add_control_address(_params: u32) -> Option<RawBytes> {
    abort!(USR_UNHANDLED_MESSAGE, "not implemented")
}

pub fn del_control_address(_params: u32) -> Option<RawBytes> {
    abort!(USR_UNHANDLED_MESSAGE, "not implemented")
}

pub fn change_peerid(_params: u32) -> Option<RawBytes> {
    abort!(USR_UNHANDLED_MESSAGE, "not implemented")
}

pub fn extend_sector_expiration(_params: u32) -> Option<RawBytes> {
    abort!(USR_UNHANDLED_MESSAGE, "not implemented")
}

pub fn terminate_sectors(_params: u32) -> Option<RawBytes> {
    abort!(USR_UNHANDLED_MESSAGE, "not implemented")
}

pub fn withdraw_miner_balance(_params: u32) -> Option<RawBytes> {
    abort!(USR_UNHANDLED_MESSAGE, "not implemented")
}

pub fn change_multiaddrs(_params: u32) -> Option<RawBytes> {
    abort!(USR_UNHANDLED_MESSAGE, "not implemented")
}
