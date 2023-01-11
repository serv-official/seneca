use codec::{Decode, Encode};
use scale_info::TypeInfo;
use frame_support::RuntimeDebug;
use scale_info::prelude::vec::Vec;
use sp_arithmetic::fixed_point::*;



#[derive(PartialEq, Eq, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub struct VerifiableCredentialObject<SchemaId, AccountId, Moment, Signature>{
    pub verifiable_credential: VerifiableCredential<SchemaId, AccountId, Moment, Signature>,
    pub registrar: AccountId,
    pub registration_date : Moment,
}
#[derive(PartialEq, Eq, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub struct VerifiableCredential<SchemaId, AccountId, Moment, Signature> {
    pub id: SchemaId,
    pub context: Vec<u8>,
    pub schema: VerifiableCredentialSchema<SchemaId, Moment, Signature>,
    pub issuer: Option<AccountId>,
    pub issuance_date: Moment,
    pub expiration_date: Option<Moment>,
    pub subject: Vec<u8>,
    pub credential_holder: Vec<u8>,
    pub signature: Signature,
}

#[derive(PartialEq, Eq, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub struct VerifiableCredentialSchema<SchemaId, Moment, Signature> {
	pub id: SchemaId,
	pub name: Vec<u8>,
	pub creator: Vec<u8>,
	pub creation_date: Moment,
	pub expiration_date: Option<Moment>,
	pub mandatory_fields: Attribute,
	pub issuer_claims: Claim,
	pub subject_claims: Claim,
	pub credential_claims: Claim,
	pub signature: Signature,
    pub nonce: u64,
}
#[derive(PartialEq, Eq, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub struct Claim{
	pub property: Vec<u8>,
	pub value: Vec<u8>,
	pub schema_id: Option<i32>,
    pub claim_type: ClaimType,
}


#[derive(PartialEq, Eq, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub enum ClaimType {
    IssuerClaim,
    SubjectClaim,
    CredentialClaim,
}

#[derive(PartialEq, Eq, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub struct Attribute {
    pub name: Vec<u8>,
    pub attribute_type: AttributeType,
}

#[derive(PartialEq, Eq, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub enum AttributeType {
    Int(i8, i16, i32, i64, i128),
    Uint(u8, u16, u32, u64, u128),
    Float(FixedI64, FixedI128, FixedU128, FixedU64),
    Hex(Vec<u8>),
    DateType(Vec<u8>),
    Base64(Vec<u8>),
}