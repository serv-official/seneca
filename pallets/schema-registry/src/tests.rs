use sp_core::{H256, Pair};
use crate::mock::*;
use frame_support::assert_ok;
use crate::types::*;
use frame_system::RawOrigin;



#[test]
fn it_works_for_create_schema() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let hash = H256::random();
		let name: String = "Alice Data".to_string();
		let creator = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let expiration_date = Timestamp::now();
		let mandatory_fields = Attribute{
			name: "name".to_string(),
			attribute_type: AttributeType::Hex("0x1234".to_string()),
		};
		let issuer_claims = Claim{
			property: b"property".to_vec(),
			value: b"value".to_vec(),
			schema_id: None,
			claim_type: ClaimType::IssuerClaim,
		};
		let subject_claims = Claim{
			property: b"property".to_vec(),
			value: b"value".to_vec(),
			schema_id: None,
			claim_type: ClaimType::SubjectClaim,
		};
		let credential_claims = Claim{
			property: b"property".to_vec(),
			value: b"value".to_vec(),
			schema_id: None,
			claim_type: ClaimType::CredentialClaim,
		};
		let account_pair = account_pair("Alice");
		let account_public = account_pair.public();
		// Encode and sign the schema message.
		let data_sig = account_pair.sign(&creator);
		let schema = VerifiableCredentialSchema {
			id: 1,
			name,
			creator,
			creation_date: Timestamp::now(),
			expiration_date: Some(expiration_date),
			mandatory_fields,
			issuer_claims,
			subject_claims,
			credential_claims,
			nonce: 1,
			signature: data_sig,
		};
		// Dispatch a signed create schema extrinsic.
		assert_ok!(SchemaRegistry::create_schema(RawOrigin::Root.into(), hash, name, creator, 
												mandatory_fields, Some(expiration_date), issuer_claims, 
												subject_claims, credential_claims, data_sig));
		// Read pallet storage and assert an expected result.
		assert_eq!(SchemaRegistry::schema_registry(hash), Some(schema.clone()));
	});
}

#[test]
fn it_works_for_update_schema() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let hash = H256::random();
		let name: String = "Alice Data".to_string();
		let creator = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let creator2 = b"did:serv:XHDx7jPsiED6n47eNfERrB5BRHZb59jVW6UMZZMTSBpikzvh".to_vec();
		let expiration_date = Timestamp::now();
		let mandatory_fields = Attribute{
			name: "name".to_string(),
			attribute_type: AttributeType::Hex("0x1234".to_string()),
		};
		let issuer_claims = Claim{
			property: b"property".to_vec(),
			value: b"value".to_vec(),
			schema_id: None,
			claim_type: ClaimType::IssuerClaim,
		};
		let subject_claims = Claim{
			property: b"property".to_vec(),
			value: b"value".to_vec(),
			schema_id: None,
			claim_type: ClaimType::SubjectClaim,
		};
		let credential_claims = Claim{
			property: b"property".to_vec(),
			value: b"value".to_vec(),
			schema_id: None,
			claim_type: ClaimType::CredentialClaim,
		};
		let account_pair = account_pair("Alice");
		let account_public = account_pair.public();
		// Encode and sign the schema message.
		let data_sig = account_pair.sign(&creator);
		let data_sig2 = account_pair.sign(&creator);
		let schema = VerifiableCredentialSchema {
			id: 1,
			name,
			creator,
			creation_date: Timestamp::now(),
			expiration_date: Some(expiration_date),
			mandatory_fields,
			issuer_claims,
			subject_claims,
			credential_claims,
			nonce: 1,
			signature: data_sig,
		};
		let schema2 = VerifiableCredentialSchema {
			id: 1,
			name,
			creator,
			creation_date: Timestamp::now(),
			expiration_date: Some(expiration_date),
			mandatory_fields,
			issuer_claims,
			subject_claims,
			credential_claims,
			nonce: 2,
			signature: data_sig2,
		};

		// Dispatch a signed extrinsic.
		assert_ok!(SchemaRegistry::create_schema(RawOrigin::Root.into(), hash, name, creator, 
												mandatory_fields, Some(expiration_date), issuer_claims, 
												subject_claims, credential_claims, data_sig));
		assert_ok!(SchemaRegistry::update_schema(RawOrigin::Root.into(), hash, schema2));
		// Read pallet storage and assert an expected result.
		assert_eq!(SchemaRegistry::schema_registry(hash), Some(schema2.clone()));
	});
}

#[test]
fn it_works_for_delete_schema() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let hash = H256::random();
		let name: String = "Alice Data".to_string();
		let creator = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let expiration_date = Timestamp::now();
		let mandatory_fields = Attribute{
			name: "name".to_string(),
			attribute_type: AttributeType::Hex("0x1234".to_string()),
		};
		let issuer_claims = Claim{
			property: b"property".to_vec(),
			value: b"value".to_vec(),
			schema_id: None,
			claim_type: ClaimType::IssuerClaim,
		};
		let subject_claims = Claim{
			property: b"property".to_vec(),
			value: b"value".to_vec(),
			schema_id: None,
			claim_type: ClaimType::SubjectClaim,
		};
		let credential_claims = Claim{
			property: b"property".to_vec(),
			value: b"value".to_vec(),
			schema_id: None,
			claim_type: ClaimType::CredentialClaim,
		};
		let account_pair = account_pair("Alice");
		let account_public = account_pair.public();
		// Encode and sign the schema message.
		let data_sig = account_pair.sign(&creator);
		let schema = VerifiableCredentialSchema {
			id: 1,
			name,
			creator,
			creation_date: Timestamp::now(),
			expiration_date: Some(expiration_date),
			mandatory_fields,
			issuer_claims,
			subject_claims,
			credential_claims,
			nonce: 1,
			signature: data_sig,
		};
		// Dispatch a signed create schema extrinsic.
		assert_ok!(SchemaRegistry::create_schema(RawOrigin::Root.into(), hash, name, creator, 
												mandatory_fields, Some(expiration_date), issuer_claims, 
												subject_claims, credential_claims, data_sig));
		// Dispatch a signed extrinsic.
		assert_ok!(SchemaRegistry::delete_schema(RawOrigin::Root.into(), hash.clone()));
		// Read pallet storage and assert an expected result.
		assert_eq!(SchemaRegistry::schema_registry(hash), None);
	});
}
