//! Benchmarking setup for pallet-kitties

use super::*;

#[allow(unused)]
use crate::Pallet as PalletModule;
use frame_benchmarking::{whitelisted_caller, benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;

benchmarks! {
	create_kitty {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))
}

impl_benchmark_test_suite!(
    PalletModule,
    crate::tests::new_test_ext(),
    crate::tests::Test,
);