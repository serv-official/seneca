use crate::types::*;
use frame_support::{dispatch::DispatchResult};
use scale_info::prelude::vec::Vec;

pub trait Schema<AccountId, Moment, Signature, Hash> {
    fn create_verifiable_schema(
        name: &Vec<u8>,
        creator: &AccountId,
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
        random_hash: &Hash,
    ) -> DispatchResult;
    fn create_verifiable_credential(
        context: &Vec<u8>,
        schema: &Vec<u8>,
        issuer: &AccountId,
        issuance_date: Option<Moment>,
        expiration_date: Option<Moment>,
        subject: &Subject,
        credential_holder: &Vec<u8>,
        signature: &Signature,
        nonce: &u64,
        random_hash: &Hash,
    ) -> DispatchResult;
    fn update_verifiable_schema(
        old_schema_key: &Hash, 
        new_data: &(Signature, VerifiableCredentialSchema<AccountId, Moment>),
    ) -> DispatchResult;
    fn update_verifiable_credential(
        old_credential_sig: &Hash, 
        new_data: &(Signature, VerifiableCredential<AccountId, Moment>)
    ) -> DispatchResult;
    fn delete_verifiable_schema(
        key: &Hash,
    ) -> DispatchResult;
    fn delete_verifiable_credential(
        key: &Hash,
    ) -> DispatchResult;
    fn is_valid_signer(data: &[u8], sig: &Signature, from: &AccountId) -> DispatchResult;
}