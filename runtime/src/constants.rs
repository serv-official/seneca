//! A set of constant values used in substrate runtime.

/// Money matters.
pub mod currency {
	use node_primitives::Balance;

	// Unit = the base number of indivisible units for balances
	pub const ZNO: Balance = 1_000 * MILLIZNO;
	pub const MILLIZNO: Balance = 1_000 * MICROZNO;
	pub const MICROZNO: Balance = 1_000 * NANOZNO;
	pub const NANOZNO: Balance = 1_000 * PICOZNO;
	pub const PICOZNO: Balance = 1;

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 20 * ZNO + (bytes as Balance) * 100 * MICROZNO
	}
}