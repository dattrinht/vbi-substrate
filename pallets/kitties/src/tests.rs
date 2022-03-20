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
