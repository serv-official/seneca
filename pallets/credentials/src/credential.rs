use crate::types::*;
use frame_support::pallet_prelude::DispatchError;
use frame_support::{dispatch::DispatchResult};
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
        new_data: &(Signature, VerifiableCredential<Moment>)
    ) -> DispatchResult;
    fn delete_verifiable_credential(
        key: &CredentialId,
    ) -> DispatchResult;
    fn is_valid_signer(data: &[u8], sig: &Signature, from: &AccountId) -> DispatchResult;
    fn split_publickey_from_did(did: &Vec<u8>) -> Result<AccountId, DispatchError>;
    fn get_credentials_by_schemaid(schema_id: &u32) -> Vec<(CredentialId, VerifiableCredential<Moment>)>;
    fn get_credentials_by_subject(subject: &Subject) -> Vec<(CredentialId, VerifiableCredential<Moment>)>;
    fn get_credentials_by_holder(holder: &Vec<u8>) -> Vec<(CredentialId, VerifiableCredential<Moment>)>;
    fn get_credentials_by_creator(creator: &Vec<u8>) -> Vec<(CredentialId, VerifiableCredential<Moment>)>;
}