pub mod balances;
pub mod runtime;
pub mod support;
pub mod system;

use runtime::Runtime;
use support::Dispatch;

mod types {
	use crate::{support, RuntimeCall};

	pub type AccountId = String;
	pub type Balance = u128;
	pub type Nonce = u32;
	pub type BlockNumber = u32;
	pub type Extrinsic = support::Extrinsic<AccountId, RuntimeCall>;
	pub type Header = support::Header<BlockNumber>;
	pub type Block = support::Block<Header, Extrinsic>;
}

#[derive(Debug)]
pub enum RuntimeCall {
	BalanceTransfer { to: types::AccountId, amount: types::Balance },
}

impl Runtime {
	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		for (i, types::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
			self.system.inc_block_number();
			dbg!(block.header.block_number, self.system.block_number());
			if block.header.block_number != self.system.block_number() {
				return Err("Block number mismatch");
			};
			self.system.inc_nonce(&caller);
			dbg!(&call, &caller);
			println!(">> Extrinsic: {:?};Block: {};", &call, self.system.block_number());
			let _res = self.dispatch(caller, call).map_err(|e| {
				eprintln!(
					"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
					block.header.block_number, i, e
				)
			});
		}
		Ok(())
	}
}

impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;

	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
		match runtime_call {
			RuntimeCall::BalanceTransfer { to, amount } => {
				self.balances.transfer(&caller, &to, amount)?;
			},
		}

		unimplemented!();
	}
}

fn main() {
	let alice = &"alice".to_string();
	let bob = &"bob".to_string();
	let charlie = &"charlie".to_string();

	let mut runtime = runtime::Runtime::new();
	runtime.balances.set_balance(alice, 100);

	println!("{alice}'s balance: {}", runtime.balances.balance(alice));

	// emulating block
	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1); // remove when extrinsic is implemented

	runtime.system.inc_nonce(alice);
	let _transfer_res = runtime.balances.transfer(alice, bob, 30).map_err(|e| {
		println!("alice to bob transfer failed for 30 token");
		eprintln!("{}", e);
	});
	println!(">> transfer from {alice} to {bob} for 30 token");

	println!("{alice}'s balance: {}", runtime.balances.balance(alice));
	println!("{bob}'s balance: {}", runtime.balances.balance(bob));

	runtime.system.inc_nonce(alice);
	let _transfer_res = runtime.balances.transfer(alice, charlie, 20).map_err(|e| {
		println!("{alice} to charlie transfer failed for 20 token");
		eprintln!("{}", e);
	});

	println!(">> transfer from {alice} to {charlie} for 20 token");

	println!("{alice}'s balance: {}", runtime.balances.balance(alice));
	println!("{charlie}'s balance: {}", runtime.balances.balance(charlie));
}
