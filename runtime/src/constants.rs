//! A set of constant values used in substrate runtime.

/// Money matters.
pub mod currency {
	use node_primitives::Balance;

	// Unit = the base number of indivisible units for balances
	pub const MILLICENTS: Balance = 10u128.pow(9);
	pub const CENTS: Balance = 10u128.pow(12); // assume this is worth about a cent.
	pub const DOLLARS: Balance = 10u128.pow(15);

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 56 * CENTS + (bytes as Balance) * 50 * MILLICENTS
	}
}

/// Fee-related.
pub mod fee {
	use frame_support::weights::{
		constants::ExtrinsicBaseWeight, WeightToFeeCoefficient, WeightToFeeCoefficients,
		WeightToFeePolynomial,
	};
	use node_primitives::Balance;
	use smallvec::smallvec;
	pub use sp_runtime::Perbill;

	/// Handles converting a weight scalar to a fee value, based on the scale and granularity of the
	/// node's balance type.
	///
	/// This should typically create a mapping between the following ranges:
	///   - [0, `MAXIMUM_BLOCK_WEIGHT`]
	///   - [Balance::min, Balance::max]
	///
	/// Yet, it can be used for any other sort of change to weight-fee. Some examples being:
	///   - Setting it to `0` will essentially disable the weight fee.
	///   - Setting it to `1` will cause the literal `#[weight = x]` values to be charged.
	pub struct WeightToFee;
	impl WeightToFeePolynomial for WeightToFee {
		type Balance = Balance;
		fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
			// in seneca, extrinsic base weight (smallest non-zero weight) is mapped to 1/10 CENT:
			let p = super::currency::CENTS / 10;
			let q = 100 * Balance::from(ExtrinsicBaseWeight::get().ref_time());
			smallvec![WeightToFeeCoefficient {
				degree: 1,
				negative: false,
				coeff_frac: Perbill::from_rational(p % q, q),
				coeff_integer: p / q,
			}]
		}
	}
}
