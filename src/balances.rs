use std::collections::BTreeMap;

pub struct Pallet {
	balances: BTreeMap<String, u128>,
}

impl Pallet {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn new_with_data(data: Vec<(String, u128)>) -> Self {
		let mut balances = BTreeMap::new();

		data.into_iter().for_each(|data_i| {
			balances.insert(data_i.0, data_i.1);
		});

		Self { balances }
	}

	pub fn get_balance(&self, who: &String) -> Option<u128> {
		self.balances.get(who).copied()
	}

	pub fn set_balance(&mut self, who: &String, amount: u128) {
		self.balances.insert(who.clone(), amount);
	}
	pub fn balance(&self, who: &String) -> u128 {
		*self.balances.get(who).unwrap_or(&0)
	}

	pub fn transfer(
		&mut self,
		from: &String,
		to: &String,
		amount: u128,
	) -> Result<(), &'static str> {
		let from_balance = self.balance(from);

		let to_balance = self.balance(to);

		let updated_from = from_balance.checked_sub(amount).ok_or("Insufficient balance")?;

		let updated_to = to_balance.checked_add(from_balance - updated_from).ok_or("Overflow")?;

		self.set_balance(from, updated_from);
		self.set_balance(to, updated_to);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use crate::balances::Pallet;

	#[test]
	fn init_balances() {
		let mut balances = Pallet::new();

		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}

	#[test]
	fn transfer_balance() {
		let mut balances = Pallet::new();

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
