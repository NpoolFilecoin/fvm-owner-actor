use thiserror::Error;
use fvm_shared::address::Address;

pub enum NftError {
}

/// Transfer nft to somebody then also transfer the amount benefit to target user
pub fn transfer_nft(
    to: &Address,
    // token id
) -> Result<(), NftError> {
    Ok(())
}

/// Cancel nft transfer within timeout
pub fn cancel_transfer_nft(
    // token id
) -> Result<(), NftError> {
    Ok(())
}

/// Approve nft from somebody then get tbe amount benefit
pub fn approve_nft(
    // token id
) -> Result<(), NftError> {
    Ok(())
}

