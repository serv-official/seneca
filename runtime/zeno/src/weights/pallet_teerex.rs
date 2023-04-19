//! Autogenerated weights for `pallet_teerex`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-11-11, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/zeno-node
// benchmark
// --chain
// dev
// --steps=50
// --repeat=20
// --pallet=pallet_teerex
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=runtime/src/weights/pallet_teerex.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_teerex.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_teerex::WeightInfo for WeightInfo<T> {
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Teerex AllowSGXDebugMode (r:1 w:0)
	// Storage: Teerex EnclaveIndex (r:1 w:0)
	// Storage: Teerex EnclaveRegistry (r:0 w:1)
	fn register_enclave() -> Weight {
		Weight::from_ref_time(2_087_072_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}

	// TODO benchmark dcap registration
	fn register_dcap_enclave() -> Weight {
		Weight::from_ref_time(1_969_500_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// TODO benchmark dcap registration
	fn register_quoting_enclave() -> Weight {
		Weight::from_ref_time(1_969_500_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	
	// Storage: Teerex EnclaveIndex (r:1 w:2)
	// Storage: Teerex EnclaveCount (r:1 w:1)
	// Storage: Teerex EnclaveRegistry (r:1 w:2)
	fn unregister_enclave() -> Weight {
		Weight::from_ref_time(94_173_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	fn call_worker() -> Weight {
		Weight::from_ref_time(54_902_000)
	}
	// Storage: Teerex EnclaveIndex (r:1 w:0)
	fn confirm_processed_parentchain_block() -> Weight {
		Weight::from_ref_time(52_350_000)
			.saturating_add(T::DbWeight::get().reads(1))
	}

	fn publish_hash() -> Weight {
		Weight::from_ref_time(1_969_500_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
