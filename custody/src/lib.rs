#[macro_use]
extern crate abort;

use fvm_ipld_encoding::RawBytes;
use fvm_ipld_encoding::Cbor;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_shared::address::Address;

use cid::Cid;
use thiserror::Error;

use miner::{Miner, MinerError};
use beneficiary::{PercentBeneficiary, AmountBeneficiary};
use params::deserialize;

#[derive(Error, Debug)]
pub enum CustodyError {
    #[error("miner call error {0}")]
    MinerCallError(#[from] MinerError),
}

pub fn custody_miner(
    miner_id: &Address,
    power_actor_state: &Cid,
    percent_beneficiaries: Vec<PercentBeneficiary>,
    amount_beneficiaries: Vec<AmountBeneficiary>,
) -> Result<Miner, CustodyError> {
    let mut miner = Miner::from_id(miner_id);
    match miner.initialize_info(power_actor_state) {
        Ok(_) => {},
        Err(err) => return Err(CustodyError::MinerCallError(err)),
    }
    match miner.set_percent_beneficiaries(percent_beneficiaries) {
        Ok(_) => {},
        Err(err) => return Err(CustodyError::MinerCallError(err)),
    };
    match miner.set_amount_beneficiaries(amount_beneficiaries) {
        Ok(_) => {},
        Err(err) => return Err(CustodyError::MinerCallError(err)),
    }
    Ok(miner)
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
