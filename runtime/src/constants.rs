//! A set of constant values used in substrate runtime.

/// Money matters.
pub mod currency {
	use node_primitives::Balance;

	pub const MILLISER: Balance = 100_000_000;
	pub const SER: Balance = MILLISER / 1_000; // assume this is worth about a cent.
	pub const ZNO: Balance = SER / 100;

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 15 * SER + (bytes as Balance) * 6 * SER
	}
}