#![cfg(test)]

use crate::{mock::*, *};
use frame_support::{assert_ok};

// In mock.rs, we've created 2 kitties in genesis:
// a Female and Male owned by account 1 and 2, respectively.

// This function checks that kitty ownership is set correctly in storage.
// This will panic if things are not correct.
fn assert_ownership(owner: u64, kitty_id: sp_core::H256) {
	// For a kitty to be owned it should exist.
	let kitty = Kitties::<Test>::get(kitty_id).unwrap();
	// The kitty's owner is set correctly.
	assert_eq!(kitty.owner, owner);

	for (check_owner, owned) in KittiesOwned::<Test>::iter() {
		if owner == check_owner {
			// Owner should have this kitty.
			assert!(owned.contains(&kitty_id));
		} else {
			// Everyone else should not.
			assert!(!owned.contains(&kitty_id));
		}
	}
}

#[test]
fn create_kitty_should_work() {
	new_test_ext(vec![
		(1, *b"1234567890123456", Gender::Female),
		(2, *b"123456789012345a", Gender::Male),
	])
	.execute_with(|| {
		// create a kitty with account #10
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(10)));

		// check that 3 kitties exists (together with the 2 from genesis)
		assert_eq!(KittyCnt::<Test>::get(), 3);

		// check that account #10 owns 1 kitty
		let kitties_owned = KittiesOwned::<Test>::get(10);
		assert_eq!(kitties_owned.len(), 1);
		let id = kitties_owned.last().unwrap();
		assert_ownership(10, *id);

		// check that this kitty is specifically owned by account #10
		let kitty = Kitties::<Test>::get(id).unwrap();
		assert_eq!(kitty.owner, 10);
		assert_eq!(kitty.price, None);
	});
}

#[test]
fn transfer_kitty_should_work() {
	new_test_ext(vec![
		(1, *b"1234567890123456", Gender::Female),
		(2, *b"123456789012345a", Gender::Male),
	])
	.execute_with(|| {
		// check that account 10 own a kitty
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(10)));
		let id = KittiesOwned::<Test>::get(10)[0];

		// account 10 send kitty to account 3
		assert_ok!(SubstrateKitties::transfer(Origin::signed(10), 3, id));

		// account 10 now has nothing
		assert_eq!(KittiesOwned::<Test>::get(10).len(), 0);
		// but account 3 does
		assert_eq!(KittiesOwned::<Test>::get(3).len(), 1);
		assert_ownership(3, id);
	});
}

#[test]
fn buy_kitty_should_work() {
	new_test_ext(vec![
		(1, *b"1234567890123456", Gender::Female),
		(2, *b"123456789012345a", Gender::Male),
	])
	.execute_with(|| {
		// Account #2 sets a price of 4 for their kitty
		let id = KittiesOwned::<Test>::get(2)[0];
		let set_price = 4;

		assert_ok!(SubstrateKitties::set_price(Origin::signed(2), id, Some(set_price)));

		// Account #1 can buy account #2's kitty
		assert_ok!(SubstrateKitties::buy_kitty(Origin::signed(1), id, set_price));
	});
}

#[test]
fn set_price_should_work() {
	new_test_ext(vec![
		(1, *b"1234567890123456", Gender::Female),
		(2, *b"123456789012345a", Gender::Male),
	])
	.execute_with(|| {
		// New price is set to 4
		let id = KittiesOwned::<Test>::get(2)[0];
		let set_price = 4;
		assert_ok!(SubstrateKitties::set_price(Origin::signed(2), id, Some(set_price)));
	});
}

#[test]
fn breed_kitty_should_work() {
	new_test_ext(vec![
		(1, *b"1234567890123456", Gender::Female),
		(2, *b"123456789012345a", Gender::Male),
	])
	.execute_with(|| {
		// Mint female kitty for account #1
		let mom_dna = [0u8; 16];
		let mom = SubstrateKitties::mint(&1, Some(mom_dna), Some(Gender::Female)).unwrap();

		// Mint male kitty for account #1
		let dad_dna = [1u8; 16];
		let dad = SubstrateKitties::mint(&1, Some(dad_dna), Some(Gender::Male)).unwrap();

		// Breeder can only breed kitties they own
		assert_ok!(SubstrateKitties::breed_kitty(Origin::signed(1), mom, dad));
	});
}