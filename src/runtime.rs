use crate::{
	balances, proof_of_existence, system,
	types::{AccountId, Balance, BlockNumber, Content, Nonce},
};

#[derive(Debug)]
pub struct Runtime {
	pub system: system::Pallet<Self>,
	pub balances: balances::Pallet<Self>,
	pub pallet_existence: proof_of_existence::Pallet<Self>,
}

impl Runtime {
	pub fn new() -> Self {
		Self {
			system: system::Pallet::new(),
			balances: balances::Pallet::new(),
			pallet_existence: proof_of_existence::Pallet::new(),
		}
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

impl proof_of_existence::Config for Runtime {
	type Content = Content;
}
