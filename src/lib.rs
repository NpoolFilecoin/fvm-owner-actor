#[macro_use]
mod abort;

mod custody;
use crate::custody::*;

mod deposit;
use crate::deposit::*;

mod nft;
use crate::nft::*;

mod upgrade;
use crate::upgrade::*;

mod setting;
use crate::setting::*;

mod withdraw;
use crate::withdraw::*;

mod sealing;
use crate::sealing::*;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use fvm_shared::METHOD_CONSTRUCTOR;
use fvm_sdk::NO_DATA_BLOCK_ID;
use fvm_ipld_encoding::{DAG_CBOR, RawBytes};

#[derive(FromPrimitive)]
#[repr(u64)]
pub enum Method {
    Constructor = METHOD_CONSTRUCTOR,

    /// Custody miner method
    CustodyMiner,
    ChangeWorkerAddress,
    AddControlAddress,
    DelControlAddress,
    ChangePeerID,
    ExtendSectorExpiration,
    TerminateSectors,
    WithdrawMinerBalance,
    ChangeMultiaddrs,

    /// Legacy method: add deposit
    ///  It help to submit the deposit account and amount before we can detect method 0 invocation to myself
    AddDeposit,

    /// Legacy method: vote to start or stop sealing
    SealingVote,

    /// NFT method: nft will be minted when deposit happen, and transfered between user
    ///  When nft transfered, the relevant amount ration will be transfered, too
    ///  For safe transfer, user has to send / approve then the NFT will be transfered
    ///  Approved of nft should be delayed, send nft could be canceled within timeout
    TransferNFT,
    CancelTransferNFT,
    ApproveNFT,

    /// Upgrade method
    UpgradeInitialize,
    UpgradeVote,
    UpgradeFinalize,

    /// Withdraw method
    WithdrawBalance,
}

#[no_mangle]
pub fn invoke(params: u32) -> u32 {
    let method = fvm_sdk::message::method_number();
    let ret: Option<RawBytes> = match FromPrimitive::from_u64(method) {
        Some(Method::Constructor) => constructor(params),

        // Custody miner method
        Some(Method::CustodyMiner) => custody_miner(params),
        Some(Method::ChangeWorkerAddress) => change_worker_address(params),
        Some(Method::AddControlAddress) => add_control_address(params),
        Some(Method::DelControlAddress) => del_control_address(params),
        Some(Method::ChangePeerID) => change_peerid(params),
        Some(Method::ExtendSectorExpiration) => extend_sector_expiration(params),
        Some(Method::TerminateSectors) => terminate_sectors(params),
        Some(Method::WithdrawMinerBalance) => withdraw_miner_balance(params),
        Some(Method::ChangeMultiaddrs) => change_multiaddrs(params),

        // Legacy deposit method
        Some(Method::AddDeposit) => add_deposit(params),

        // Legacy sealing method
        Some(Method::SealingVote) => sealing_vote(params),

        // NFT method
        Some(Method::TransferNFT) => transfer_nft(params),
        Some(Method::CancelTransferNFT) => cancel_transfer_nft(params),
        Some(Method::ApproveNFT) => approve_nft(params),

        // Upgrade method
        Some(Method::UpgradeInitialize) => upgrade_initialize(params),
        Some(Method::UpgradeVote) => upgrade_vote(params),
        Some(Method::UpgradeFinalize) => upgrade_finalize(params),

        // Withdraw method
        Some(Method::WithdrawBalance) => withdraw_balance(params),

        _ => abort!(USR_UNHANDLED_MESSAGE, "unrecognized method"),
    };

    match ret {
        None => NO_DATA_BLOCK_ID,
        Some(v) => match fvm_sdk::ipld::put_block(DAG_CBOR, v.bytes()) {
            Ok(id) => id,
            Err(err) => abort!(USR_SERIALIZATION, "failed to store return value: {}", err),
        },
    }
}

/// Constructor method with initial setting of the actor
pub fn constructor(params: u32) -> Option<RawBytes> {
    setting_initialize(params)?;
    sealing_initialize(params)
}

#[cfg(test)]
mod tests {
}
