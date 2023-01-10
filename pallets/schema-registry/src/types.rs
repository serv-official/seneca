use chrono::prelude::*;
use chrono::{DateTime, TimeZone, Utc};
use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
use scale_info::prelude::vec::Vec;
use scale_info::TypeInfo;
use sp_core::sr25519;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(PartialEq, Eq, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub struct Signature {
	sigtype: String,
	nonce: String,
	value: sr25519,
}
/// schema registry structure
#[derive(PartialEq, Eq, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub struct Registry {
	pub id: u64,
	pub address: sr25519::Public,
	pub registry_type: RegistryType,
	pub data: Vec<u8>,
}


pub enum RegistryType {
	CredentialRegsitry,
	VerifiableCredential,
}

#[derive(PartialEq, Eq, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub struct VerifiableCredential {
	context: String,
	schema: String,
	issuer: String,
	issuance_date: SystemTime,
	expiration_date: SystemTime,
	subject: String,
	credential_holder: String,
	signature: Signature,
}
