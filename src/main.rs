pub mod balances;
pub mod runtime;
pub mod system;

fn main() {
	let alice = &"alice".to_string();
	let bob = &"bob".to_string();
	let charlie = &"charlie".to_string();

	let mut runtime = runtime::Runtime::new();
	runtime.balances.set_balance(alice, 100);

	println!("{alice}'s balance: {}", runtime.balances.balance(alice));

	// emulating block
	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);

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
