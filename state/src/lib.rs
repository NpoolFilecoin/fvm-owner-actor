#[macro_use]
extern crate abort;

use std::collections::HashMap;

use fvm_sdk as sdk;
use fvm_shared::address::Address;

use fvm_ipld_encoding::DAG_CBOR;
use fvm_ipld_encoding::to_vec;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_ipld_encoding::CborStore;

use cid::multihash::Code;
use cid::Cid;

use blockstore::Blockstore;
use miner::Miner;
use beneficiary::AmountBeneficiary;
use upgrade::Upgrade;
use sealing::Sealing;

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
pub struct State {
    pub miners: HashMap<Address, Miner>,
    pub deposits: Vec<AmountBeneficiary>,
    pub upgrade: Option<Upgrade>,
    pub sealing: Sealing,
}

impl State {
    pub fn load() -> Self {
        // First, load the current state root.
        let root = match sdk::sself::root() {
            Ok(root) => root,
            Err(err) => abort!(USR_ILLEGAL_STATE, "failed to get root: {:?}", err),
        };

        // Load the actor state from the state tree.
        match Blockstore.get_cbor::<Self>(&root) {
            Ok(Some(state)) => state,
            Ok(None) => abort!(USR_ILLEGAL_STATE, "state does not exist"),
            Err(err) => abort!(USR_ILLEGAL_STATE, "failed to get state: {}", err),
        }
    }

    pub fn save(&self) -> Cid {
        let serialized = match to_vec(self) {
            Ok(s) => s,
            Err(err) => abort!(USR_SERIALIZATION, "failed to serialize state: {:?}", err),
        };
        let cid = match sdk::ipld::put(Code::Blake2b256.into(), 32, DAG_CBOR, serialized.as_slice())
        {
            Ok(cid) => cid,
            Err(err) => abort!(USR_SERIALIZATION, "failed to store initial state: {:}", err),
        };
        if let Err(err) = sdk::sself::set_root(&cid) {
            abort!(USR_ILLEGAL_STATE, "failed to set root ciid: {:}", err);
        }
        cid
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            miners: HashMap::new(),
            deposits: Vec::new(),
            upgrade: None,
            sealing: Sealing {
                approved_percent: 100,
                rejected_percent: 0,
            },
        }
    }
}
