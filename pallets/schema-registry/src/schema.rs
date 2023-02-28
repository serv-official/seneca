use crate::types::*;

use frame_support::dispatch::DispatchResult;

pub trait Schema<AccountId, Moment, Signature> {
    fn create_verifiable_schema(
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
        context: &Vec<u8>,
        schema: &Vec<u8>,
        issuer: &Vec<u8>,
        issuance_date: Option<Moment>,
        expiration_date: Option<Moment>,
        subject: &Subject,
        credential_holder: &Vec<u8>,
        signature: &Signature,
        nonce: &u64,
    ) -> DispatchResult;
    fn update_verifiable_schema(
        old_schema_key: &Signature, 
        new_data: &VerifiableCredentialSchema<Moment>,
    ) -> DispatchResult;
    fn update_verifiable_credential(
        old_credential_sig: &Signature, 
        new_data: &VerifiableCredential<Moment>
    ) -> DispatchResult;
    fn delete_verifiable_schema(
        key: &Signature,
    ) -> DispatchResult;
    fn delete_verifiable_credential(
        key: &Signature,
    ) -> DispatchResult;
    fn is_valid_signer(
        signature: &Signature,
        msg: &[u8],
        signer: &AccountId,
    ) -> DispatchResult;
    fn split_string(string: &str) -> &str;
}