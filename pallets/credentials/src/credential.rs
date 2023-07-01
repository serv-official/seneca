use crate::types::*;
use frame_support::dispatch::DispatchResult;
use scale_info::prelude::vec::Vec;

pub trait Credential<AccountId, Moment, Signature, CredentialId> {
	fn create_verifiable_credential(
		id: &CredentialId,
		context: &Vec<u8>,
		schema: &u32,
		issuer: &Vec<u8>,
		issuance_date: Option<Moment>,
		expiration_date: Option<Moment>,
		subject: &Subject,
		credential_holder: &Vec<u8>,
		signature: &Signature,
		nonce: &u64,
	) -> DispatchResult;
	fn update_verifiable_credential(
		old_credential_sig: &CredentialId,
		new_data: &(Signature, VerifiableCredential<Moment>),
	) -> DispatchResult;
	fn delete_verifiable_credential(key: &CredentialId) -> DispatchResult;
	fn get_credentials_by_schemaid(
		schema_id: &u32,
	) -> Vec<(CredentialId, VerifiableCredential<Moment>)>;
	fn get_credentials_by_subject(
		subject: &Subject,
	) -> Vec<(CredentialId, VerifiableCredential<Moment>)>;
	fn get_credentials_by_holder(
		holder: &Vec<u8>,
	) -> Vec<(CredentialId, VerifiableCredential<Moment>)>;
	fn get_credentials_by_creator(
		creator: &Vec<u8>,
	) -> Vec<(CredentialId, VerifiableCredential<Moment>)>;
}
