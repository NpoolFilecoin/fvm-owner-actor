use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_shared::address::Address;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SealingError {
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
pub struct Sealing {
    pub approved_percent: u32,
    pub rejected_percent: u32,
}

pub fn sealing_vote(
    miner_id: &Address,
    approved: bool,
) -> Result<(), SealingError> {
    Ok(())
}
