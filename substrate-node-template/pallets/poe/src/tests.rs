use crate::{mock::*, Dog, Error, Proofs};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let dog = Dog::<u8> { name: vec![1], color: vec![1], dog_id: vec![1], age: 2 };
		assert_ok!(Poe::create_claim(Origin::signed(1), vec![1], vec![1], 2, vec![1]));
		// Read pallet storage and assert an expected result.
		assert_eq!(
			Proofs::<Test>::get(vec![1]),
			(1, frame_system::Pallet::<Test>::block_number(), dog)
		);
	});
}

#[test]
fn create_claim_error() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let dog = Dog::<u8> { name: vec![1], color: vec![1], dog_id: vec![1], age: 2 };
		assert_ok!(Poe::create_claim(Origin::signed(1), vec![1], vec![1], 2, vec![1]));
		// Read pallet storage and assert an expected result.
		assert_noop!(
			Poe::create_claim(Origin::signed(1), vec![1], vec![1], 2, vec![1]),
			Error::<Test>::ProofAlreadyClaimed
		);
	});
}
