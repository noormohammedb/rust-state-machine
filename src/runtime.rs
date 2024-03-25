use crate::{
	balances, system,
	types::{AccountId, Balance, BlockNumber, Nonce},
};

#[derive(Debug)]
pub struct Runtime {
	pub system: system::Pallet<Self>,
	pub balances: balances::Pallet<Self>,
}

impl Runtime {
	pub fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}

impl system::Config for Runtime {
	type AccountId = AccountId;
	type BlockNumber = BlockNumber;
	type Nonce = Nonce;
}

impl balances::Config for Runtime {
	type Balance = Balance;
}
