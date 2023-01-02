//! A set of constant values used in substrate runtime.

/// Money matters.
pub mod currency {
	use node_primitives::Balance;

	pub const MILLISER: Balance = 1_000_000_000;
	pub const SER: Balance = 1_000 * MILLISER; // assume this is worth about a cent.
	pub const SERV: Balance = 100 * SER;

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 15 * SER + (bytes as Balance) * 6 * SER
	}
}