use sp_core::Pair;
use crate::mock::*;
use frame_support::assert_ok;
use crate::types::*;
use frame_system::RawOrigin;
use sp_runtime::traits::IdentifyAccount;



#[test]
fn it_works_for_create_schema() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let name = b"Alice Data".to_vec();
		let creator = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let expiration_date = Timestamp::now();
		let mandatory_fields = Attribute{
			name: b"name".to_vec(),
			attribute_type: AttributeType::Hex,
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
		let account_id = account_key("Alice");
		// Encode and sign the schema message.
		let data_sig = account_pair.sign(&creator);
		// Dispatch a signed create schema extrinsic.
		assert_ok!(SchemaRegistry::create_schema(RawOrigin::Signed(account_id).into(), 0, name.clone(), creator.clone(), 
												vec![mandatory_fields.clone()], Some(expiration_date), issuer_claims.clone(), 
												subject_claims.clone(), credential_claims.clone(), data_sig.clone()));

	});

}

#[test]
fn it_works_for_create_credential() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let name = b"Alice Data".to_vec();
		let creator = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let context = b"Credential context".to_vec();
		let expiration_date = Timestamp::now();
		let mandatory_fields = Attribute{
			name: b"name".to_vec(),
			attribute_type: AttributeType::Hex,
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
		let account_pub = account_key("Alice");
		let account_id = account_pub.into_account();
		
		let subject = b"Credential subject".to_vec();
		let credential_holder = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		// Encode and sign the schema message.
		let data_sig = account_pair.sign(&creator);
		let schema = VerifiableCredentialSchema {
			id: 0,
			name: name.clone(),
			creator: creator.clone(),
			creation_date: Timestamp::now(),
			expiration_date: Some(expiration_date),
			mandatory_fields: vec![mandatory_fields.clone()],
			issuer_claims: issuer_claims.clone(),
			subject_claims: subject_claims.clone(),
			credential_claims: credential_claims.clone(),
			nonce: 2,
			signature: data_sig.clone(),
		};

		// Dispatch a signed create schema extrinsic.
		assert_ok!(SchemaRegistry::create_credential(RawOrigin::Signed(account_pub).into(), 0, context.clone(), schema.clone(), 
													Some(account_id), Some(1702379816u64), 
													subject.clone(), credential_holder.clone(),data_sig.clone()));

	});

}

#[test]
fn it_works_for_update_schema() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let name = b"Alice Data".to_vec();
		let creator = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let expiration_date = Timestamp::now();
		let mandatory_fields = Attribute{
			name: b"name".to_vec(),
			attribute_type: AttributeType::Hex,
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
		let account_pub = account_key("Alice");
		// Encode and sign the schema message.
		let data_sig = account_pair.sign(&creator);
		let schema = VerifiableCredentialSchema {
			id: 0,
			name: name.clone(),
			creator: creator.clone(),
			creation_date: Timestamp::now(),
			expiration_date: Some(expiration_date),
			mandatory_fields: vec![mandatory_fields.clone()],
			issuer_claims: issuer_claims.clone(),
			subject_claims: subject_claims.clone(),
			credential_claims: credential_claims.clone(),
			nonce: 2,
			signature: data_sig.clone(),
		};

		// Dispatch a signed extrinsic.
		assert_ok!(SchemaRegistry::create_schema(RawOrigin::Signed(account_pub).into(), 0, name, creator, 
												vec![mandatory_fields], Some(expiration_date), issuer_claims, 
												subject_claims, credential_claims, data_sig));
		assert_ok!(SchemaRegistry::update_schema(RawOrigin::Root.into(), 0, schema.clone()));
		assert_eq!(SchemaRegistry::schema_registry(0), Some(schema));

	})
}

#[test]
fn it_works_for_update_credential() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let name = b"Alice Data".to_vec();
		let creator = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let context = b"Credential context".to_vec();
		let expiration_date = Timestamp::now();
		let mandatory_fields = Attribute{
			name: b"name".to_vec(),
			attribute_type: AttributeType::Hex,
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
		let account_pub = account_key("Alice");
		let account_id = account_pub.into_account();
		
		let subject = b"Credential subject".to_vec();
		let credential_holder = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		// Encode and sign the schema message.
		let data_sig = account_pair.sign(&creator);
		let schema = VerifiableCredentialSchema {
			id: 0,
			name: name.clone(),
			creator: creator.clone(),
			creation_date: Timestamp::now(),
			expiration_date: Some(expiration_date),
			mandatory_fields: vec![mandatory_fields.clone()],
			issuer_claims: issuer_claims.clone(),
			subject_claims: subject_claims.clone(),
			credential_claims: credential_claims.clone(),
			nonce: 2,
			signature: data_sig.clone(),
		};

		let credential = VerifiableCredential{
				id: 1,
				context: context.clone(),
				schema: schema.clone(),
				issuer: Some(account_id),
				issuance_date: Timestamp::now(),
				expiration_date: Some(1702379816u64),
				subject: subject.clone(),
				credential_holder: credential_holder.clone(),
				signature: data_sig.clone(),
		};

		// Dispatch a signed create schema extrinsic.
		assert_ok!(SchemaRegistry::create_credential(RawOrigin::Signed(account_pub).into(), 0, context.clone(), schema.clone(), 
													Some(account_id), Some(1702379816u64), 
													subject.clone(), credential_holder.clone(),data_sig.clone()));
		assert_ok!(SchemaRegistry::update_credential(RawOrigin::Signed(account_pub).into(), 0, credential.clone()));
		assert_eq!(SchemaRegistry::credential_registry(0), Some(credential));

	})
}

#[test]
fn it_works_for_delete_schema() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let name = b"Alice Data".to_vec();
		let creator = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let expiration_date = Timestamp::now();
		let mandatory_fields = Attribute{
			name: b"name".to_vec(),
			attribute_type: AttributeType::Hex,
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
		let account_pub = account_key("Alice");
		// Encode and sign the schema message.
		let data_sig = account_pair.sign(&creator);
		// Dispatch a signed create schema extrinsic.
		assert_ok!(SchemaRegistry::create_schema(RawOrigin::Signed(account_pub).into(), 0, name, creator, 
												vec![mandatory_fields], Some(expiration_date), issuer_claims, 
												subject_claims, credential_claims, data_sig));
		// Dispatch a signed extrinsic.
		assert_ok!(SchemaRegistry::delete_schema(RawOrigin::Signed(account_pub).into(), 0));
		// Read pallet storage and assert an expected result.
		assert_eq!(SchemaRegistry::schema_registry(0), None);
	});
}

#[test]
fn it_works_for_delete_credential() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let name = b"Alice Data".to_vec();
		let creator = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let context = b"Credential context".to_vec();
		let expiration_date = Timestamp::now();
		let mandatory_fields = Attribute{
			name: b"name".to_vec(),
			attribute_type: AttributeType::Hex,
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
		let account_pub = account_key("Alice");
		let account_id = account_pub.into_account();
		
		let subject = b"Credential subject".to_vec();
		let credential_holder = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		// Encode and sign the schema message.
		let data_sig = account_pair.sign(&creator);
		let schema = VerifiableCredentialSchema {
			id: 0,
			name: name.clone(),
			creator: creator.clone(),
			creation_date: Timestamp::now(),
			expiration_date: Some(expiration_date),
			mandatory_fields: vec![mandatory_fields.clone()],
			issuer_claims: issuer_claims.clone(),
			subject_claims: subject_claims.clone(),
			credential_claims: credential_claims.clone(),
			nonce: 2,
			signature: data_sig.clone(),
		};

		// Dispatch a signed create schema extrinsic.
		assert_ok!(SchemaRegistry::create_credential(RawOrigin::Signed(account_pub).into(), 0, context.clone(), schema.clone(), 
													Some(account_id), Some(1702379816u64), 
													subject.clone(), credential_holder.clone(),data_sig.clone()));
		// Dispatch a signed extrinsic.
		assert_ok!(SchemaRegistry::delete_credential(RawOrigin::Signed(account_pub).into(), 0));
		// Read pallet storage and assert an expected result.
		assert_eq!(SchemaRegistry::credential_registry(0), None);
	});
}