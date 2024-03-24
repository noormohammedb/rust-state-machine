use num::{CheckedAdd, One, Unsigned, Zero};
use std::{collections::BTreeMap, ops::AddAssign};

#[derive(Debug)]
pub struct Pallet<BlockNumber, AccountId, Nonce> {
	block_number: BlockNumber,
	nonce: BTreeMap<AccountId, Nonce>,
}

impl<BlockNumber, AccountId, Nonce> Pallet<BlockNumber, AccountId, Nonce>
where
	BlockNumber: Zero + One + CheckedAdd + Unsigned + AddAssign + Copy,
	AccountId: Ord + Clone,
	Nonce: Zero + One + Unsigned + Copy,
{
	pub fn new() -> Self {
		Self { block_number: BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	pub fn _genesis_init(b_num: BlockNumber, nonce_data: Vec<(AccountId, Nonce)>) -> Self {
		let mut nonce = BTreeMap::new();
		nonce_data.into_iter().for_each(|nonce_i| {
			nonce.insert(nonce_i.0, nonce_i.1);
		});
		Self { block_number: b_num, nonce }
	}

	pub fn block_number(&self) -> BlockNumber {
		self.block_number
	}

	pub fn inc_block_number(&mut self) -> BlockNumber {
		let new_block_num = self.block_number + BlockNumber::one();
		self.block_number += BlockNumber::one();

		new_block_num
	}

	pub fn nonce(&self, who: &AccountId) -> Nonce {
		self.nonce.get(who).copied().unwrap_or(Nonce::zero())
	}

	pub fn inc_nonce(&mut self, who: &AccountId) {
		let current = self.nonce.get(who).copied().unwrap_or(Nonce::zero());
		self.nonce.insert(who.clone(), current + Nonce::one());
	}
}

#[cfg(test)]
mod tests {
	use super::Pallet;

	#[test]
	fn init_system() {
		let mut system = Pallet::<u32, String, u32>::new();
		let alice = &"Alice".to_string();

		assert_eq!(system.block_number(), 0);
		assert_eq!(system.nonce(alice), 0);

		system.inc_block_number();
		system.inc_nonce(alice);

		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce(alice), 1);
	}
}
