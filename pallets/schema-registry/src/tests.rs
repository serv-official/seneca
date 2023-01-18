use codec::Encode;
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
		let claim = Claim{
			property: b"property".to_vec(),
			value: b"value".to_vec(),
			schema_id: None,
			claim_type: ClaimType::IssuerClaim,
		};
		let account_pair = account_pair("Alice");
		let account_id = account_key("Alice");
		// Encode and sign the schema message.
		let schema = VerifiableCredentialSchema {
			name: name.clone(),
			creator: creator.clone(),
			public: false, 
			creation_date: Timestamp::now(),
			expiration_date: Some(expiration_date),
			mandatory_fields: vec![mandatory_fields.clone()],
			issuer_claims: claim.clone(),
			subject_claims: claim.clone(),
			credential_claims: claim.clone(),
			nonce: 2,
		};
		let data_sig = account_pair.sign(&schema.encode());
		// Dispatch a signed create schema extrinsic.
		assert_ok!(SchemaRegistry::create_schema(RawOrigin::Signed(account_id).into(), name.clone(), creator.clone(), false,  
												vec![mandatory_fields.clone()], Some(expiration_date), claim.clone(), 
												claim.clone(), claim.clone(), data_sig.clone()));

	});

}

#[test]
fn it_works_for_create_credential() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let context = b"Credential context".to_vec();
		let claim = Claim{
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
		let schema = "verifiableCredentialSchema".encode();
		let credential = VerifiableCredential{
			context: context.clone(),
			schema: schema.clone(),
			issuer: Some(account_id),
			claim: claim.clone(),
			issuance_date: Timestamp::now(),
			expiration_date: Some(1702379816u64),
			subject: subject.clone(),
			credential_holder: credential_holder.clone(),
		};
		let data_sig = account_pair.sign(&credential.encode());
		// Dispatch a signed create schema extrinsic.
		assert_ok!(SchemaRegistry::create_credential(RawOrigin::Signed(account_pub).into(), context.clone(), schema.clone(), 
													Some(account_id), claim.clone(),  Some(1702379816u64), 
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
		let claim = Claim{
			property: b"property".to_vec(),
			value: b"value".to_vec(),
			schema_id: None,
			claim_type: ClaimType::IssuerClaim,
		};
		let account_pair = account_pair("Alice");
		let account_pub = account_key("Alice");
		// Encode and sign the schema message.
		let schema = VerifiableCredentialSchema {
			name: name.clone(),
			creator: creator.clone(),
			public: false,
			creation_date: Timestamp::now(),
			expiration_date: Some(expiration_date),
			mandatory_fields: vec![mandatory_fields.clone()],
			issuer_claims: claim.clone(),
			subject_claims: claim.clone(),
			credential_claims: claim.clone(),
			nonce: 2,
		};
		let data_sig = account_pair.sign(&schema.encode());
		// Dispatch a signed extrinsic.
		assert_ok!(SchemaRegistry::create_schema(RawOrigin::Signed(account_pub).into(), name, creator, false,
												vec![mandatory_fields], Some(expiration_date), claim.clone(), 
												claim.clone(), claim.clone(), data_sig.clone()));
		assert_ok!(SchemaRegistry::update_schema(RawOrigin::Root.into(), data_sig.clone(), schema.clone()));
		assert_eq!(SchemaRegistry::schema_registry(data_sig.clone()), Some(schema));

	})
}

#[test]
fn it_works_for_update_credential() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let context = b"Credential context".to_vec();
		let claim = Claim{
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
		let schema = "VerifiableCredentialSchema".encode();

		let credential = VerifiableCredential{
				context: context.clone(),
				schema: schema.clone(),
				issuer: Some(account_id),
				claim: claim.clone(),
				issuance_date: Timestamp::now(),
				expiration_date: Some(1702379816u64),
				subject: subject.clone(),
				credential_holder: credential_holder.clone(),
		};
		let data_sig = account_pair.sign(&credential.encode());
		// Dispatch a signed create schema extrinsic.
		assert_ok!(SchemaRegistry::create_credential(RawOrigin::Signed(account_pub).into(), context.clone(), schema.clone(), 
													Some(account_id), claim.clone(), Some(1702479816u64), 
													subject.clone(), credential_holder.clone(),data_sig.clone()));
		assert_ok!(SchemaRegistry::update_credential(RawOrigin::Signed(account_pub).into(), data_sig.clone(), credential.clone()));
		assert_eq!(SchemaRegistry::credential_registry(data_sig.clone()), Some(credential));

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
		let claim = Claim{
			property: b"property".to_vec(),
			value: b"value".to_vec(),
			schema_id: None,
			claim_type: ClaimType::CredentialClaim,
		};
		let account_pair = account_pair("Alice");
		let account_pub = account_key("Alice");
		let schema = VerifiableCredentialSchema {
			name: name.clone(),
			creator: creator.clone(),
			public: false,
			creation_date: Timestamp::now(),
			expiration_date: Some(expiration_date),
			mandatory_fields: vec![mandatory_fields.clone()],
			issuer_claims: claim.clone(),
			subject_claims: claim.clone(),
			credential_claims: claim.clone(),
			nonce: 1,
		};
		let data_sig = account_pair.sign(&schema.encode());
		// Dispatch a signed create schema extrinsic.
		assert_ok!(SchemaRegistry::create_schema(RawOrigin::Signed(account_pub).into(), name, creator, false, 
												vec![mandatory_fields], Some(expiration_date), claim.clone(), 
												claim.clone(), claim.clone(), data_sig.clone()));
		// Dispatch a signed extrinsic.
		assert_ok!(SchemaRegistry::delete_schema(RawOrigin::Signed(account_pub).into(), data_sig.clone()));
		// Read pallet storage and assert an expected result.
		assert_eq!(SchemaRegistry::schema_registry(data_sig.clone()), None);
	});
}

#[test]
fn it_works_for_delete_credential() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let context = b"Credential context".to_vec();
		let claim = Claim{
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
		let schema = "VerefiableCredentialSchema".encode();
		let credential = VerifiableCredential{
			context: context.clone(),
			schema: schema.clone(),
			issuer: Some(account_id),
			claim: claim.clone(),
			issuance_date: Timestamp::now(),
			expiration_date: Some(1702379816u64),
			subject: subject.clone(),
			credential_holder: credential_holder.clone(),
	};
	let data_sig = account_pair.sign(&credential.encode());
		// Dispatch a signed create schema extrinsic.
		assert_ok!(SchemaRegistry::create_credential(RawOrigin::Signed(account_pub).into(),context.clone(), schema.clone(), 
													Some(account_id), claim, Some(1702379816u64), 
													subject.clone(), credential_holder.clone(),data_sig.clone()));
		// Dispatch a signed extrinsic.
		assert_ok!(SchemaRegistry::delete_credential(RawOrigin::Signed(account_pub).into(), data_sig.clone()));
		// Read pallet storage and assert an expected result.
		assert_eq!(SchemaRegistry::credential_registry(data_sig.clone()), None);
	});
}