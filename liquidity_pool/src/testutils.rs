#![cfg(feature = "testutils")]

use soroban_sdk::{BigInt, Env, FixedBinary};
use soroban_token_contract::public_types::Identifier;

pub fn register_test_contract(e: &Env, contract_id: &[u8; 32]) {
    let contract_id = FixedBinary::from_array(e, *contract_id);
    e.register_contract(&contract_id, crate::LiquidityPool {});
}

pub use crate::deposit::invoke as deposit;
pub use crate::initialize::invoke as initialize;
pub use crate::share_id::invoke as share_id;
pub use crate::swap::invoke as swap;
pub use crate::withdraw::invoke as withdraw;

pub struct LiquidityPool {
    env: Env,
    contract_id: FixedBinary<32>,
}

impl LiquidityPool {
    pub fn new(env: &Env, contract_id: &[u8; 32]) -> Self {
        Self {
            env: env.clone(),
            contract_id: FixedBinary::from_array(env, *contract_id),
        }
    }

    pub fn initialize(&self, token_a: &[u8; 32], token_b: &[u8; 32]) {
        let token_a = FixedBinary::from_array(&self.env, *token_a);
        let token_b = FixedBinary::from_array(&self.env, *token_b);
        initialize(&self.env, &self.contract_id, &token_a, &token_b)
    }

    pub fn share_id(&self) -> FixedBinary<32> {
        share_id(&self.env, &self.contract_id)
    }

    pub fn deposit(&self, to: &Identifier) {
        deposit(&self.env, &self.contract_id, to)
    }

    pub fn swap(&self, to: &Identifier, out_a: &BigInt, out_b: &BigInt) {
        swap(&self.env, &self.contract_id, to, out_a, out_b)
    }

    pub fn withdraw(&self, to: &Identifier) {
        withdraw(&self.env, &self.contract_id, to)
    }
}
