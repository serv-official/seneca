//! Autogenerated weights for `pallet_im_online`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-27, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `MACs-MacBook-Air.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/zeno-node
// benchmark
// pallet
// --chain
// -dev
// --steps=50
// --repeat=20
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --pallet=pallet_im_online
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --output=./runtime/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_im_online`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_im_online::WeightInfo for WeightInfo<T> {
	/// Storage: Session Validators (r:1 w:0)
	/// Proof Skipped: Session Validators (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Session CurrentIndex (r:1 w:0)
	/// Proof Skipped: Session CurrentIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ImOnline Keys (r:1 w:0)
	/// Proof: ImOnline Keys (max_values: Some(1), max_size: Some(320002), added: 320497, mode: MaxEncodedLen)
	/// Storage: ImOnline ReceivedHeartbeats (r:1 w:1)
	/// Proof: ImOnline ReceivedHeartbeats (max_values: None, max_size: Some(25), added: 2500, mode: MaxEncodedLen)
	/// Storage: ImOnline AuthoredBlocks (r:1 w:0)
	/// Proof: ImOnline AuthoredBlocks (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// The range of component `k` is `[1, 1000]`.
	fn validate_unsigned_and_then_heartbeat(k: u32, _v: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `361 + k * (32 ±0)`
		//  Estimated: `321487 + k * (1761 ±0)`
		// Minimum execution time: 83_488_000 picoseconds.
		Weight::from_parts(99_862_268, 0)
			.saturating_add(Weight::from_parts(0, 321487))
			// Standard Error: 567
			.saturating_add(Weight::from_parts(35_207, 0).saturating_mul(k.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 1761).saturating_mul(k.into()))
	}
}