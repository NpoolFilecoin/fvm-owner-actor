use fvm_shared::address::Address;
use fvm_shared::econ::TokenAmount;
use fvm_shared::sector::{RegisteredPoStProof, StoragePower};
use fvm_shared::bigint::bigint_ser;
use fvm_shared::HAMT_BIT_WIDTH;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_ipld_hamt::{Hamt, Error as HamtError};
use cid::Cid;
use power::{get_power_actor_state, PowerError};
use thiserror::Error;
use blockstore::Blockstore;
use beneficiary::{PercentBeneficiary, AmountBeneficiary};

#[derive(Error, Debug)]
pub enum MinerError {
    #[error("power call error {0}")]
    PowerCallError(#[from] PowerError),
    #[error("fvm ipld hamt error {0}")]
    FvmIpldHamtError(#[from] HamtError),
    #[error("invalid miner state")]
    InvalidMinerStateError,
    #[error("invalid percent value")]
    InvalidPercentValueError,
    #[error("invalid amount value")]
    InvalidAmountValueError,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
pub struct Miner {
    pub miner_id: Address,
    pub window_post_proof_type: RegisteredPoStProof,
    pub initial_collateral: TokenAmount,
    pub initial_vesting: TokenAmount,
    pub initial_available: TokenAmount,
    #[serde(with = "bigint_ser")]
    pub initial_raw_power: StoragePower,
    #[serde(with = "bigint_ser")]
    pub initial_adj_power: StoragePower,
    /// Initial beneficiaries, e.g. hardware investor, operator, etc.
    pub percent_beneficiaries: Vec<PercentBeneficiary>,
    /// Amount beneficiaries, e.g. exist collateral investor
    pub amount_beneficiaries: Vec<AmountBeneficiary>,
}

impl Miner {
    pub fn from_id(id: &Address) -> Self {
        Self {
            miner_id: id.clone(),
            window_post_proof_type: RegisteredPoStProof::StackedDRGWindow32GiBV1,
            initial_collateral: TokenAmount::from_atto(0),
            initial_vesting: TokenAmount::from_atto(0),
            initial_available: TokenAmount::from_atto(0),
            initial_raw_power: StoragePower::from(0),
            initial_adj_power: StoragePower::from(0),
            percent_beneficiaries: Vec::new(),
            amount_beneficiaries: Vec::new(),
        }
    }

    pub fn set_percent_beneficiaries(&mut self, beneficiaries: Vec<PercentBeneficiary>) -> Result<(), MinerError> {
        self.percent_beneficiaries = beneficiaries;
        // TODO: sum of percent should be less than 100
        Ok(())
    }

    pub fn set_amount_beneficiaries(&mut self, beneficiaries: Vec<AmountBeneficiary>) -> Result<(), MinerError> {
        self.amount_beneficiaries = beneficiaries;
        // TODO: sum of amount should be less than exist collaterral
        Ok(())
    }

    pub fn initialize_info(&mut self, power_actor_state: &Cid) -> Result<(), MinerError> {
        // TODO: get miner collateral
        // TODO: get miner available balance
        // TODO: get miner vesting

        let state = match get_power_actor_state(power_actor_state) {
            Ok(state) => state,
            Err(err) => return Err(MinerError::PowerCallError(err)),
        };

        #[derive(Debug, Serialize_tuple, Deserialize_tuple, Clone, PartialEq)]
        struct Claim {
            pub window_post_proof_type: RegisteredPoStProof,
            #[serde(with = "bigint_ser")]
            pub raw_byte_power: StoragePower,
            #[serde(with = "bigint_ser")]
            pub quality_adj_power: StoragePower,
        }

        // TODO: only get what i need
        let claims = match Hamt::<Blockstore, Claim>::load_with_bit_width(&state.claims, Blockstore, HAMT_BIT_WIDTH) {
            Ok(claims) => claims,
            Err(err) => return Err(MinerError::FvmIpldHamtError(err)),
        };
        let claim = match claims.get(&self.miner_id.to_bytes()) {
            Ok(claim) => match claim {
                Some(claim) => claim,
                None => return Err(MinerError::InvalidMinerStateError),
            },
            Err(err) => return Err(MinerError::FvmIpldHamtError(err)),
        };

        self.initial_raw_power = claim.clone().raw_byte_power;
        self.initial_adj_power = claim.clone().quality_adj_power;
        self.window_post_proof_type = claim.window_post_proof_type;

        Ok(())
    }
}
