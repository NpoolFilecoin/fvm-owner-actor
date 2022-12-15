use fvm_shared::address::Address;
use fvm_shared::econ::TokenAmount;
use fvm_shared::sector::{RegisteredPoStProof, StoragePower};
use fvm_shared::bigint::bigint_ser;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
pub struct Beneficiary {
    pub address: Address,
    pub percent: u32,
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
    pub beneficiaries: Vec<Beneficiary>,
}

impl Miner {
    pub fn from_id(id: &Address) -> Self {
        // TODO: get initial setting
        Self {
            miner_id: id.clone(),
            window_post_proof_type: RegisteredPoStProof::StackedDRGWindow32GiBV1,
            initial_collateral: TokenAmount::from_atto(0),
            initial_vesting: TokenAmount::from_atto(0),
            initial_available: TokenAmount::from_atto(0),
            initial_raw_power: StoragePower::from(0),
            initial_adj_power: StoragePower::from(0),
            beneficiaries: Vec::new(),
        }
    }

    pub fn set_beneficiaries(&mut self, beneficiaries: Vec<Beneficiary>) {
        self.beneficiaries = beneficiaries;
    }
}
