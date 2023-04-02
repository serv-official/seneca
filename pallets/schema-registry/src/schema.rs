use crate::types::*;
use frame_support::pallet_prelude::DispatchError;
use frame_support::{dispatch::DispatchResult};
use scale_info::prelude::vec::Vec;

pub trait Schema<AccountId, Moment, Signature, SchemaId, CredentialId> {
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
    fn update_verifiable_schema(
        old_schema_key: &SchemaId, 
        new_data: &(Signature, VerifiableCredentialSchema<Moment>),
    ) -> DispatchResult;
    fn update_verifiable_credential(
        old_credential_sig: &CredentialId, 
        new_data: &(Signature, VerifiableCredential<Moment>)
    ) -> DispatchResult;
    fn delete_verifiable_schema(
        key: &SchemaId,
    ) -> DispatchResult;
    fn delete_verifiable_credential(
        key: &CredentialId,
    ) -> DispatchResult;
    fn is_valid_signer(data: &[u8], sig: &Signature, from: &AccountId) -> DispatchResult;
    fn split_publickey_from_did(did: &Vec<u8>) -> Result<AccountId, DispatchError>;
}