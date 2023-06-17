use crate::types::*;
use frame_support::dispatch::DispatchResult;
use frame_support::pallet_prelude::DispatchError;
use scale_info::prelude::vec::Vec;

pub trait Schema<AccountId, Moment, Signature, SchemaId> {
	fn create_verifiable_schema(
		id: &SchemaId,
		name: &Vec<u8>,
		creator: &Vec<u8>,
		public: &bool,
		creation_date: Moment,
		expiration_date: Option<Moment>,
		mandatory_fields: &Vec<Attribute>,
		issuer_claims: &Vec<Claim>,
		subject_claims: &Vec<Claim>,
		credential_claims: &Vec<Claim>,
		metadata: &Vec<u8>,
		signature: &Signature,
		nonce: &u64,
	) -> DispatchResult;
	fn update_verifiable_schema(
		old_schema_key: &SchemaId,
		new_data: &(Signature, VerifiableCredentialSchema<Moment>),
	) -> DispatchResult;
	fn delete_verifiable_schema(key: &SchemaId) -> DispatchResult;
	fn is_valid_signer(data: &[u8], sig: &Signature, from: &AccountId) -> DispatchResult;
	fn split_publickey_from_did(did: &Vec<u8>) -> Result<AccountId, DispatchError>;
}

pub trait SchemaInterface {
	type SchemaId;
	fn check_schema_id_exists(schema: Self::SchemaId) -> DispatchResult;
	fn to_schema_id(schema: &u32) -> Self::SchemaId;
}
