use codec::{Decode, Encode};
use scale_info::TypeInfo;
use frame_support::RuntimeDebug;
use scale_info::prelude::vec::Vec;



#[derive(PartialEq, Eq, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub struct VerifiableCredentialObject<AccountId, Moment>{
    pub verifiable_credential: VerifiableCredential<AccountId, Moment>,
    pub registrar: AccountId,
    pub registration_date : Moment,
}
#[derive(PartialEq, Eq, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub struct VerifiableCredential<AccountId, Moment> {
    pub context: Vec<u8>,
    pub schema: Vec<u8>,
    pub issuer: AccountId,
    pub issuance_date: Option<Moment>,
    pub expiration_date: Option<Moment>,
    pub subject: Subject,
    pub credential_holder: Vec<u8>,
    pub nonce: u64,
}

#[derive(PartialEq, Eq, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub struct VerifiableCredentialSchema<AccountId, Moment> {
	pub name: Vec<u8>,
	pub creator: AccountId,
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
#[derive(PartialEq, Eq, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub struct Claim{
    pub schemaid: Option<Vec<u8>>,
	pub property: Vec<u8>,
	pub value: Vec<u8>,
    pub claim_type: ClaimType,
    pub issuance_requirement: Option<Vec<IssuanceRequirement>>,
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
pub struct Subject {
    pub id: Vec<u8>,
    pub claim: Vec<Claim>,
}

#[derive(PartialEq, Eq, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub enum AttributeType {
    Int,
    Uint,
    Float,
    Hex,
    DateType,
    Base64,
    Text,
}

#[derive(PartialEq, Eq, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub struct IssuanceRequirement {
    pub name: Vec<u8>,
    pub insuance_type: IssuanceType,
}

#[derive(PartialEq, Eq, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub enum IssuanceType {
    Int,
    Uint,
    Float,
    Hex,
    DateType,
    Base64,
    Text,
}