use fvm_shared::address::Address;
use cid::Cid;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UpgradeError {
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
pub struct Upgrade {
    pub actor_code_id: Cid,
    pub submitter: Address,
    pub approved_percent: u32,
    pub rejected_percent: u32,
}

pub fn upgrade_initialize(
    actor_code_id: &Cid,
) -> Result<(), UpgradeError> {
    Ok(())
}

pub fn upgrade_vote(
    approved: bool,
) -> Result<(), UpgradeError> {
    Ok(())
}

pub fn upgrade_finalize() ->Result<(), UpgradeError> {
    Ok(())
}

