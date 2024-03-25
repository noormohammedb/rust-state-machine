use num::{CheckedAdd, CheckedSub, Unsigned, Zero};
use std::collections::BTreeMap;

use crate::system;

pub trait Config: system::Config {
	type Balance: Ord + Copy + Zero + CheckedAdd + CheckedSub + Unsigned + core::fmt::Debug;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn new_with_data(data: Vec<(T::AccountId, T::Balance)>) -> Self {
		let mut balances = BTreeMap::new();

		data.into_iter().for_each(|data_i| {
			balances.insert(data_i.0, data_i.1);
		});

		Self { balances }
	}

	pub fn get_balance(&self, who: &T::AccountId) -> Option<T::Balance> {
		self.balances.get(who).copied()
	}

	pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
		self.balances.insert(who.clone(), amount);
	}
	pub fn balance(&self, who: &T::AccountId) -> T::Balance {
		*self.balances.get(who).unwrap_or(&T::Balance::zero())
	}

	pub fn transfer(
		&mut self,
		from: &T::AccountId,
		to: &T::AccountId,
		amount: T::Balance,
	) -> Result<(), &'static str> {
		let from_balance = self.balance(from);

		let to_balance = self.balance(to);

		let updated_from = from_balance.checked_sub(&amount).ok_or("Insufficient balance")?;

		let updated_to =
			to_balance.checked_add(&(from_balance - updated_from)).ok_or("Overflow")?;

		self.set_balance(from, updated_from);
		self.set_balance(to, updated_to);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::{Config, Pallet};
	use crate::{
		system,
		types::{AccountId, Balance, BlockNumber, Nonce},
	};

	struct TestConfig {
		balances: Pallet<Self>,
	}

	impl TestConfig {
		fn new() -> Self {
			Self { balances: Pallet::new() }
		}
	}

	impl Config for TestConfig {
		type Balance = Balance;
	}

	impl system::Config for TestConfig {
		type AccountId = AccountId;
		type BlockNumber = BlockNumber;
		type Nonce = Nonce;
	}

	#[test]
	fn init_balances() {
		let TestConfig { mut balances } = TestConfig::new();

		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}

	#[test]
	fn transfer_balance() {
		let TestConfig { mut balances } = TestConfig::new();

		let alice = &"alice".to_string();
		let bob = &"bob".to_string();
		let insufficient_err = Err("Insufficient balance");

		assert_eq!(balances.transfer(alice, bob, 57), insufficient_err);

		balances.set_balance(alice, 45);

		assert_eq!(balances.balance(bob), 0);

		assert_eq!(balances.transfer(alice, bob, 44), Ok(()));

		assert_eq!(balances.balance(alice), 1);
		assert_eq!(balances.balance(bob), 44);
	}
}
