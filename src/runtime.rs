use crate::{
	balances, system,
	types::{AccountId, Balance, BlockNumber, Nonce},
};

#[derive(Debug)]
pub struct Runtime {
	pub system: system::Pallet<BlockNumber, AccountId, Nonce>,
	pub balances: balances::Pallet<AccountId, Balance>,
}

impl Runtime {
	pub fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}
