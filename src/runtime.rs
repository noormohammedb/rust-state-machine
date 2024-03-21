use crate::{balances, system};

pub struct Runtime {
	pub system: system::Pallet,
	pub balances: balances::Pallet,
}

impl Runtime {
	pub fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}
