use chrono::prelude::*;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
// use sp_core::sr25519::*;
//DIDData types
#[derive(PartialEq, Eq, TypeInfo, Clone, Encode, Decode, Debug)]
pub struct DIDData {
	pub id: u64,
	pub data: Vec<u8>,
}
pub struct Subject {
	pub id: u64,
}
pub struct SingatureType {
	pub sigtype: String,
	pub sigvalue: String,
	pub signonce: String,
}

pub struct VCObjects {
	pub context: String,
	pub id: u64,
	pub schema: SchemaRegistry,
	pub issuer: String,
	pub issuanceDate: DateTime<Utc>,
	pub expirationDate: DateTime<Utc>,
	pub subject: Subject,
	pub credentialHolder: String,
	pub signature: SingatureType,
}

pub struct VCobjects {
	pub did: String,
	pub vcs: [VcObjects],
}

pub struct VerifiableCredentialObject {
	VerifiableCredentialObject: [VCobjects],
}

pub struct DidDocObj {
	pub didDocument: [DidDocument],
	pub registrar: Address,
	pub registrationDate: DateTime<Utc>,
}

pub struct PublicKeys {
	pub id: u64,
	pub owner: String,
	pub keytype: String,
	pub keyHex: vec<T>,
}
pub struct Service {
	pub id: String,
	pub serviceEnpoint: String,
	pub servicetype: String,
}
pub struct Authentication {
	authtype: String,
	publicKey: vec<T>,
}

pub struct DidDocument {
	pub context: String,
	pub id: u64,
	pub publicKeys: [PublicKeys],
	pub service: [Service],
    pub authentication: Authentication,
    pub updated: DateTime<Utc>,
    pub authority: Option<String>,
}
struct VerifiableCredentialSchema {
    id: i32,
    name: String,
    creator: String,
    creation_date: DateTime<Utc>,
    expiration_date: Option<DateTime<Utc>>,
    mandatory_fields: Vec<Attribute>,
    issuer_claims: Vec<Claim>,
    subject_claims: Vec<Claim>,
    credential_claims: Vec<CredentialClaim>,
    signature: Signature,
}

struct Claim {
    property: IssuerClaimProperty,
    value: AuthoritySet,
    schema_id: Option<i32>,
}

struct CredentialClaim {
    property: String,
    value: String,
    issuance_requirement: Vec<Attribute>,
}

struct Signature {
    sigtype: String,
    nonce: String,
    value: String,
}

pub struct VerifiableCredenialSchema {
    id: u64,
    name: Vec<String>,
    creator: String,
    creation_date: String,
    expiration_date: Option<String>,
    mandatory_fields: Vec<Attribute>,
    issuer_claims: Vec<Claim>,
    subject_claims: Vec<Claim>,
    credential_claims: Vec<CredentialClaim>,
    signature: Signature,

}
