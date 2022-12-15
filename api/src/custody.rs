use fvm_ipld_encoding::RawBytes;
use fvm_ipld_encoding::Cbor;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_shared::address::Address;

use cid::Cid;

use state::State;
use params::deserialize;
use beneficiary::{PercentBeneficiary, AmountBeneficiary};

use custody;

#[derive(Serialize_tuple, Deserialize_tuple, Clone)]
pub struct CustodyMinerParams {
    pub miner_id: Address,
    /// We cannot get power actor state here, so we need pass it outside
    pub power_actor_state: Cid,
    pub percent_beneficiaries: Vec<PercentBeneficiary>,
    pub amount_beneficiaries: Vec<AmountBeneficiary>,
}
impl Cbor for CustodyMinerParams {}

pub fn custody_miner(params: u32) -> Option<RawBytes> {
    let params = match deserialize::<CustodyMinerParams>(params) {
        Ok(params) => params,
        Err(err) => abort!(USR_SERIALIZATION, "{:?}", err),
    };

    let mut state = State::load();
    match state.miners.get(&params.miner_id) {
        Some(_) => abort!(USR_ILLEGAL_STATE, "exist miner"),
        None => {},
    }

    let miner = match custody::custody_miner(
        &params.miner_id,
        &params.power_actor_state,
        params.percent_beneficiaries,
        params.amount_beneficiaries,
    ) {
        Ok(miner) => miner,
        Err(err) => abort!(USR_ILLEGAL_STATE, "{:?}", err),
    };

    state.miners.insert(params.miner_id, miner);
    state.save();

    None
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone)]
pub struct ChangeWorkerAddressParams {
    pub miner_id: Address,
    pub new_worker: Address,
}
impl Cbor for ChangeWorkerAddressParams {}

pub fn change_worker_address(params: u32) -> Option<RawBytes> {
    let params = match deserialize::<ChangeWorkerAddressParams>(params) {
        Ok(params) => params,
        Err(err) => abort!(USR_SERIALIZATION, "{:?}", err),
    };

    match custody::change_worker_address(
        &params.miner_id,
        &params.new_worker,
    ) {
        Ok(_) => None,
        Err(err) => abort!(USR_ILLEGAL_STATE, "{:?}", err),
    }
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
