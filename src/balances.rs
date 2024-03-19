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
}
