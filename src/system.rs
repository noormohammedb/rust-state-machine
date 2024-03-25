use num::{CheckedAdd, One, Unsigned, Zero};
use std::{collections::BTreeMap, ops::AddAssign};

pub trait Config {
	type BlockNumber: Zero + One + CheckedAdd + Unsigned + AddAssign + Copy;
	type AccountId: Ord + Clone;
	type Nonce: Copy + Zero + One;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	block_number: T::BlockNumber,
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	pub fn _genesis_init(b_num: T::BlockNumber, nonce_data: Vec<(T::AccountId, T::Nonce)>) -> Self {
		let mut nonce = BTreeMap::new();
		nonce_data.into_iter().for_each(|nonce_i| {
			nonce.insert(nonce_i.0, nonce_i.1);
		});
		Self { block_number: b_num, nonce }
	}

	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	pub fn inc_block_number(&mut self) -> T::BlockNumber {
		let new_block_num = self.block_number + T::BlockNumber::one();
		self.block_number += T::BlockNumber::one();

		new_block_num
	}

	pub fn nonce(&self, who: &T::AccountId) -> T::Nonce {
		self.nonce.get(who).copied().unwrap_or(T::Nonce::zero())
	}

	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		let current = self.nonce.get(who).copied().unwrap_or(T::Nonce::zero());
		self.nonce.insert(who.clone(), current + T::Nonce::one());
	}
}

#[cfg(test)]
mod tests {
	use super::{Config, Pallet};
	use crate::types::{AccountId, BlockNumber, Nonce};

	#[test]
	fn init_system() {
		struct TestConfig {
			system: Pallet<Self>,
		}

		impl TestConfig {
			fn new() -> Self {
				Self { system: Pallet::new() }
			}
		}

		impl Config for TestConfig {
			type AccountId = AccountId;
			type BlockNumber = BlockNumber;
			type Nonce = Nonce;
		}

		let TestConfig { mut system } = TestConfig::new();
		let alice = &"Alice".to_string();

		assert_eq!(system.block_number(), 0);
		assert_eq!(system.nonce(alice), 0);

		system.inc_block_number();
		system.inc_nonce(alice);

		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce(alice), 1);
	}
}
