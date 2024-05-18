use core::fmt::Debug;
use std::collections::BTreeMap;

use crate::support::DispatchResult;

pub trait Config: crate::system::Config {
	type Content: Debug + Ord;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	content: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { content: BTreeMap::new() }
	}

	pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
		self.content.get(claim)
	}

	pub fn create_claim(&mut self, owner: T::AccountId, claim: T::Content) -> DispatchResult {
		if self.content.contains_key(&claim) {
			return Err("Claim already exist");
		}

		self.content.insert(claim, owner);

		Ok(())
	}

	pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		let current_claim = self.get_claim(&claim).ok_or("Claim not exist")?;

		if current_claim != &caller {
			return Err("this content is owned by someone else");
		}

		self.content.remove_entry(&claim);

		Ok(())
	}
}

#[derive(Debug)]
pub enum Call<T: Config> {
	CreateClaim { claim: T::Content },
	RevokeClaim { claim: T::Content },
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
	type Caller = T::AccountId;
	type Call = Call<T>;

	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
		match call {
			Call::CreateClaim { claim } => {
				return self.create_claim(caller, claim);
			},
			Call::RevokeClaim { claim } => {
				return self.revoke_claim(caller, claim);
			},
		}
	}
}

#[cfg(test)]
mod tests {
	struct TestConfig;

	impl super::Config for TestConfig {
		type Content = &'static str;
	}

	impl crate::system::Config for TestConfig {
		type AccountId = &'static str;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn basic_proof_of_existence() {
		let mut poe = super::Pallet::<TestConfig>::new();
		assert_eq!(poe.get_claim(&"none_claim"), None);
		assert_eq!(poe.get_claim(&"claim_01"), None);

		assert_eq!(poe.create_claim(&"alice", &"claim_01"), Ok(()));
		assert_eq!(poe.get_claim(&"claim_01"), Some(&"alice"));
		assert_eq!(poe.create_claim(&"bob", &"claim_02"), Ok(()));
		assert_eq!(poe.get_claim(&"claim_02"), Some(&"bob"));
		assert_eq!(poe.revoke_claim(&"bob", &"claim_02"), Ok(()));

		assert_eq!(poe.create_claim(&"alice", &"claim_01"), Err("Claim already exist"));
		assert_eq!(poe.create_claim(&"bob", &"claim_01"), Err("Claim already exist"));
		assert_eq!(poe.revoke_claim(&"charlie", "none_claim"), Err("Claim not exist"));
		assert_eq!(
			poe.revoke_claim(&"bob", &"claim_01"),
			Err("this content is owned by someone else")
		);
	}
}
