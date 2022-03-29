#![cfg_attr(not(feature = "std"), no_std)]

sp_api::decl_runtime_apis! {
	pub trait KittiesRuntimeApi
	{
        fn get_kitties_count() -> u64;
	}
}