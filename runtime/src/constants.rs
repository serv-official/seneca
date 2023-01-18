//! A set of constant values used in substrate runtime.

/// Money matters.
pub mod currency {
	use node_primitives::Balance;

	pub const MILLISER: Balance = 1_000_000;
	pub const SER: Balance = MILLISER / 1_000; // assume this is worth about a cent.
	pub const ZNO: Balance = SER / 10;

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 20 * SER + (bytes as Balance) * 100 * SER
	}
}