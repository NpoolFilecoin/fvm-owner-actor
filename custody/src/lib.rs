#[macro_use]
extern crate abort;

use fvm_ipld_encoding::RawBytes;
use fvm_ipld_encoding::Cbor;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_shared::address::Address;
use libp2p::PeerId;
use fvm_shared::econ::TokenAmount;
use multiaddr::Multiaddr;

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

pub fn change_worker_address(
    miner_id: &Address,
    new_worker: &Address,
) -> Result<(), CustodyError> {
    Ok(())
}

pub fn add_control_address(
    miner_id: &Address,
    new_control: &Address,
) -> Result<(), CustodyError> {
    Ok(())
}

pub fn del_control_address(
    miner_id: &Address,
    del_control: &Address,
) -> Result<(), CustodyError> {
    Ok(())
}

pub fn change_peerid(
    miner_id: &Address,
    peer_id: &PeerId,
) -> Result<(), CustodyError> {
    Ok(())
}

pub fn extend_sector_expiration(_params: u32) -> Option<RawBytes> {
    abort!(USR_UNHANDLED_MESSAGE, "not implemented")
}

pub fn terminate_sectors(_params: u32) -> Option<RawBytes> {
    abort!(USR_UNHANDLED_MESSAGE, "not implemented")
}

pub fn withdraw_miner_balance(
    miner_id: &Address,
    amount: TokenAmount,
) -> Result<(), CustodyError> {
    Ok(())
}

pub fn change_multiaddrs(
    miner_id: &Address,
    multiaddrs: Vec<Multiaddr>,
) -> Result<(), CustodyError> {
    Ok(())
}
