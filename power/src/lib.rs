use fvm_shared::bigint::bigint_ser;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_shared::econ::TokenAmount;
use fvm_shared::sector::StoragePower;
use fvm_shared::clock::ChainEpoch;
use fvm_shared::smooth::FilterEstimate;
use fvm_ipld_encoding::CborStore;
use cid::Cid;
use thiserror::Error;
use anyhow::Error as AnyhowError;

use blockstore::Blockstore;

#[derive(Error, Debug)]
pub enum PowerError {
    #[error("invalid state")]
    InvalidStateError,
    #[error("power actor state error: {0}")]
    PowerActorStateError(#[from] AnyhowError),
}

#[derive(Default, Serialize_tuple, Deserialize_tuple)]
pub struct PowerActorState {
    #[serde(with = "bigint_ser")]
    pub total_raw_byte_power: StoragePower,
    #[serde(with = "bigint_ser")]
    pub total_bytes_committed: StoragePower,
    #[serde(with = "bigint_ser")]
    pub total_quality_adj_power: StoragePower,
    #[serde(with = "bigint_ser")]
    pub total_qa_bytes_committed: StoragePower,

    pub total_pledge_collateral: TokenAmount,

    #[serde(with = "bigint_ser")]
    pub this_epoch_raw_byte_power: StoragePower,
    #[serde(with = "bigint_ser")]
    pub this_epoch_quality_adj_power: StoragePower,

    pub this_epoch_pledge_collateral: TokenAmount,
    pub this_epoch_qa_power_smoothed: FilterEstimate,
    pub miner_count: i64,
    pub miner_above_min_power_count: i64,
    pub cron_event_queue: Cid,
    pub first_cron_epoch: ChainEpoch,
    pub claims: Cid,
    pub proof_validation_batch: Option<Cid>,
}

pub fn get_power_actor_state(state_cid: &Cid) -> Result<PowerActorState, PowerError> {
    match Blockstore.get_cbor::<PowerActorState>(&state_cid) {
        Ok(state) => match state {
            Some(state) => Ok(state),
            None => Err(PowerError::InvalidStateError),
        },
        Err(err) => Err(PowerError::PowerActorStateError(err)),
    }
}
