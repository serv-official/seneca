use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
use scale_info::prelude::vec::Vec;
use scale_info::TypeInfo;

#[derive(PartialEq, Eq, PartialOrd, Ord, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub struct VerifiableCredentialSchema<Moment> {
	pub name: Vec<u8>,
	pub creator: Vec<u8>,
	pub public: bool,
	pub creation_date: Moment,
	pub expiration_date: Option<Moment>,
	pub mandatory_fields: Vec<Attribute>,
	pub issuer_claims: Vec<Claim>,
	pub subject_claims: Vec<Claim>,
	pub credential_claims: Vec<Claim>,
	pub metadata: Vec<u8>,
	pub nonce: u64,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub struct Claim {
	pub schemaid: Option<u32>,
	pub property: Vec<u8>,
	pub value: Vec<u8>,
	pub claim_type: ClaimType,
	pub issuance_requirement: Option<Vec<IssuanceRequirement>>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub enum ClaimType {
	IssuerClaim,
	SubjectClaim,
	CredentialClaim,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub struct Attribute {
	pub name: Vec<u8>,
	pub attribute_type: AttributeType,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub struct Subject {
	pub id: Vec<u8>,
	pub claim: Vec<Claim>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub enum AttributeType {
	Int,
	Uint,
	Float,
	Hex,
	DateType,
	Base64,
	Text,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub struct IssuanceRequirement {
	pub name: Vec<u8>,
	pub insuance_type: IssuanceType,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub enum IssuanceType {
	Int,
	Uint,
	Float,
	Hex,
	DateType,
	Base64,
	Text,
}
