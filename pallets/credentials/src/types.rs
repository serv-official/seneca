use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
use pallet_schemas::types::Claim;
use scale_info::prelude::vec::Vec;
use scale_info::TypeInfo;

#[derive(PartialEq, Eq, PartialOrd, Ord, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub struct VerifiableCredentialObject<Public, Moment> {
	pub verifiable_credential: VerifiableCredential<Moment>,
	pub registrar: Public,
	pub registration_date: Moment,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub struct VerifiableCredential<Moment> {
	pub context: Vec<u8>,
	pub schema: u32,
	pub issuer: Vec<u8>,
	pub issuance_date: Option<Moment>,
	pub expiration_date: Option<Moment>,
	pub subject: Subject,
	pub credential_holder: Vec<u8>,
	pub nonce: u64,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub struct Subject {
	pub id: Vec<u8>,
	pub claim: Vec<Claim>,
}
