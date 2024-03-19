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

	pub fn get_balance(&self, key: String) -> Option<u128> {
		self.balances.get(&key).copied()
	}
}
