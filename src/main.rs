pub mod balances;
pub mod proof_of_existence;
pub mod runtime;
pub mod support;
pub mod system;

use runtime::Runtime;
use support::Dispatch;

use crate::types::{Block, Extrinsic, Header};

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
	Balances(balances::Call<Runtime>),
}

impl Runtime {
	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		for (i, types::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
			if block.header.block_number != self.system.block_number() {
				return Err("Block number mismatch");
			};
			self.system.inc_nonce(&caller);
			println!(">> Extrinsic ({caller}): {:?};Block: {}", &call, self.system.block_number());
			let _res = self.dispatch(caller, call).map_err(|e| {
				eprintln!(
					"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
					block.header.block_number, i, e
				)
			});
		}
		self.system.inc_block_number();
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
			RuntimeCall::Balances(call) => {
				return self.balances.dispatch(caller, call);
			},
		}
	}
}

fn main() {
	let alice = &"alice".to_string();
	let bob = &"bob".to_string();
	let charlie = &"charlie".to_string();

	let mut runtime = runtime::Runtime::new();

	// genesis state
	runtime.balances.set_balance(&"test00".to_owned(), 100);
	runtime.balances.set_balance(alice, 100);

	println!("{alice}'s balance: {}", runtime.balances.balance(alice));
	println!("{bob}'s balance: {}", runtime.balances.balance(bob));
	println!("{charlie}'s balance: {}", runtime.balances.balance(charlie));

	let block_00 = Block {
		header: Header { block_number: 0 },
		extrinsics: vec![Extrinsic {
			caller: String::from("test00"),
			call: RuntimeCall::Balances(balances::Call::Transfer {
				to: String::from("test01"),
				amount: 1,
			}),
		}],
	};

	runtime.execute_block(block_00).expect("invalid block");

	let alice_to_bob_30 = Extrinsic {
		caller: alice.clone(),
		call: RuntimeCall::Balances(balances::Call::Transfer { to: bob.clone(), amount: 30 }),
	};

	let alise_to_charlie_20 = Extrinsic {
		caller: alice.clone(),
		call: RuntimeCall::Balances(balances::Call::Transfer { to: charlie.clone(), amount: 20 }),
	};

	let block_01 = Block {
		header: Header { block_number: 1 },
		extrinsics: vec![alice_to_bob_30, alise_to_charlie_20],
	};

	runtime.execute_block(block_01).expect("invalid block");

	println!("{alice}'s balance: {}", runtime.balances.balance(alice));
	println!("{bob}'s balance: {}", runtime.balances.balance(bob));
	println!("{charlie}'s balance: {}", runtime.balances.balance(charlie));

	println!("{:#?}", runtime);
}
