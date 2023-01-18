//! A set of constant values used in substrate runtime.

/// Money matters.
pub mod currency {
	use node_primitives::Balance;

	pub const MILLISER: Balance = 1_000;
	pub const SER: Balance = 1 * MILLISER; // assume this is worth about a cent.
	pub const ZNO: Balance = 1 * SER;

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 10 * SER + (bytes as Balance) * 7 * SER
	}
}