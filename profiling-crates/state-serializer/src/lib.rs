use revm::{
    database::{CacheDB, EmptyDBTyped, StateBuilder},
    state::{AccountInfo, Bytecode},
    primitives::FixedBytes,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use alloy_primitives::{U256 as AlloyU256, Address as AlloyAddress, Bytes as AlloyBytes};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableAccount {
    pub nonce: String,
    pub balance: String,
    pub code: String,
    pub storage: HashMap<String, String>,
}

/// Wrapper struct that implements DeserializeOwned for State
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableState {
    pub accounts: HashMap<String, SerializableAccount>,
}

impl SerializableState {
    /// Convert to revm State
    pub fn into_state(self) -> impl std::fmt::Debug + revm::database::Database {
        let cache_db = CacheDB::new(EmptyDBTyped::<std::convert::Infallible>::default());
        let mut state = StateBuilder::new_with_database(cache_db).build();

        for (addr, account) in self.accounts {
            state.insert_account(
                AlloyAddress::from_str(&addr).expect("Invalid address hex").into(),
                AccountInfo {
                    balance: AlloyU256::from_str(&account.balance).expect("Invalid balance hex").into(),
                    nonce: AlloyU256::from_str(&account.nonce).expect("Invalid nonce hex").as_limbs()[0],
                    code: Some(Bytecode::new_raw({
                        if account.code.is_empty() || account.code == "0x" {
                            Vec::new()
                        } else {
                            AlloyBytes::from_str(&account.code).expect("Invalid code hex").to_vec()
                        }
                    }.into())),
                    code_hash: FixedBytes::from_slice(&[0u8; 32]),
                },
            );
        }

        state
    }
}


