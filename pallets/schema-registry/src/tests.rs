use codec::Encode;
use sp_core::Pair;
use sp_runtime::traits::IdentifyAccount;
use crate::mock::*;
use frame_support::assert_ok;
use crate::types::*;
use frame_system::RawOrigin;



#[test]
fn it_works_for_create_schema() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let name = b"Alice Data".to_vec();
		let expiration_date = Timestamp::now();
		let mandatory_fields = Attribute{
			name: b"name".to_vec(),
			attribute_type: AttributeType::Hex,
		};
		let issuance_req = IssuanceRequirement{
			name: b"insuance".to_vec(),
			insuance_type: IssuanceType::Int,
		};
		let claim = Claim{
			property: b"property".to_vec(),
			value: b"value".to_vec(),
			schemaid: None,
			claim_type: ClaimType::IssuerClaim,
			issuance_requirement: Some(vec![issuance_req.clone()]),
		};
		let account_pair = account_pair("Alice");
		let signer = account_pair.public();
		let account_id = format!("did:seneca:{}",account_pair.public().into_account());
		let nonce = 2u64;
		let creation_date =  Timestamp::now();
		// Encode and sign the schema message.
		let schema = VerifiableCredentialSchema {
			name: name.clone(),
			creator: account_id.clone().into(),
			public: false, 
			creation_date: creation_date.clone(),
			expiration_date: Some(expiration_date),
			mandatory_fields: vec![mandatory_fields.clone()],
			issuer_claims: vec![claim.clone()],
			subject_claims: vec![claim.clone()],
			credential_claims: vec![claim.clone()],
			metadata: b"metadata".to_vec(),
			nonce,
		};
		let schema_id = 0u32;
		//dbg!("Schema: {:?}", schema);
		let binding = schema.encode();
		let vc_bytes = binding.as_slice();
		let data_sig = account_pair.sign(&vc_bytes);
		// Dispatch a signed create schema extrinsic.
		assert_ok!(SchemaRegistry::create_schema(RawOrigin::Signed(signer).into(),  schema_id, name.clone(), account_id.clone().into(), false,  
												vec![mandatory_fields.clone()], creation_date.clone(), Some(expiration_date), vec![claim.clone()], 
												vec![claim.clone()], vec![claim.clone()], b"metadata".to_vec(), data_sig.clone(), nonce));

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
			schemaid: None,
			claim_type: ClaimType::CredentialClaim,
			issuance_requirement: None,
		};
		let account_pair = account_pair("Alice");
		let signer = account_pair.public();
		println!("Signer: {}", signer);
		let account_id = format!("did:seneca:{}",account_pair.public().to_string());
		println!("account_id: {}", account_id.clone());
		println!("account_id bytes: {:#?}", account_id.clone().encode());
		let subject = Subject{
				id: b"did:seneca:5GFEtniprMeFuh8HcoVrWxz4aQtv6T5V9bkENSnfPYhY4p8H".to_vec(),
				claim: vec![claim.clone()],
		};
		let credential_holder = b"did:seneca:5GFEtniprMeFuh8HcoVrWxz4aQtv6T5V9bkENSnfPYhY4p8H".to_vec();
		let nonce = 2u64;
		// Encode and sign the schema message.
		let schema = 123456u32;
		let issuance_date =  Some(Timestamp::now());
		let credential = VerifiableCredential{
			context: context.clone(),
			schema: schema.clone(),
			issuer: account_id.clone().into(),
			issuance_date: issuance_date.clone(),
			expiration_date: Some(1702379816u64),
			subject: subject.clone(),
			credential_holder: credential_holder.clone(),
			nonce: nonce.clone(),
		};
		let schema_id = 0u32;
		let data_sig = account_pair.sign(&credential.encode());
		dbg!("data_sig: {:?}", data_sig.clone());
		// Dispatch a signed create schema extrinsic.
		assert_ok!(SchemaRegistry::create_credential(RawOrigin::Signed(signer).into(), schema_id,  context.clone(), schema, 
													account_id.into(), issuance_date.clone(), Some(1702379816u64),subject.clone(),credential_holder.clone(),data_sig.clone(), nonce));

	});

}

#[test]
fn it_works_for_update_schema() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let name = b"Alice Data".to_vec();
		let expiration_date = Timestamp::now();
		let mandatory_fields = Attribute{
			name: b"name".to_vec(),
			attribute_type: AttributeType::Hex,
		};
		let issuance_req = IssuanceRequirement{
			name: b"insuance".to_vec(),
			insuance_type: IssuanceType::Int,
		};
		let claim = Claim{
			property: b"property".to_vec(),
			value: b"value".to_vec(),
			schemaid: None,
			claim_type: ClaimType::IssuerClaim,
			issuance_requirement: Some(vec![issuance_req.clone()]),
		};
		let account_pair = account_pair("Alice");
		let signer = account_pair.public();
		let account_id = format!("did:seneca:{}",account_pair.public().into_account());
		let nonce = 2u64;
		let creation_date = Timestamp::now();
		// Encode and sign the schema message.
		let schema = VerifiableCredentialSchema {
			name: name.clone(),
			creator: account_id.clone().into(),
			public: false,
			creation_date: creation_date.clone(),
			expiration_date: Some(expiration_date),
			mandatory_fields: vec![mandatory_fields.clone()],
			issuer_claims: vec![claim.clone()],
			subject_claims: vec![claim.clone()],
			credential_claims: vec![claim.clone()],
			metadata: b"metadata".to_vec(),
			nonce,
		};
		let updated_schema = VerifiableCredentialSchema {
			name: name.clone(),
			creator: account_id.clone().into(),
			public: false,
			creation_date: creation_date.clone(),
			expiration_date: Some(expiration_date),
			mandatory_fields: vec![mandatory_fields.clone()],
			issuer_claims: vec![claim.clone()],
			subject_claims: vec![claim.clone()],
			credential_claims: vec![claim.clone()],
			metadata: b"metadata2".to_vec(),
			nonce,
		};
		let data_sig = account_pair.sign(&schema.encode());
		let updated_sig = account_pair.sign(&updated_schema.encode());
		let schema_id = 0u32;
		// Dispatch a signed extrinsic.
		assert_ok!(SchemaRegistry::create_schema(RawOrigin::Signed(signer).into(), schema_id, name, account_id.clone().into(), false,
												vec![mandatory_fields], creation_date.clone(), Some(expiration_date), vec![claim.clone()], 
												vec![claim.clone()], vec![claim.clone()], b"metadata".to_vec(), data_sig.clone(), nonce));
		assert_ok!(SchemaRegistry::update_schema(RawOrigin::Signed(signer).into(), schema_id, (updated_sig.clone(), updated_schema.clone())));
		assert_eq!(SchemaRegistry::schema_registry(schema_id.clone()), Some((updated_sig, updated_schema)));

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
			schemaid: None,
			claim_type: ClaimType::CredentialClaim,
			issuance_requirement: None,
		};
		let account_pair = account_pair("Alice");
		let signer = account_pair.public();
		let account_id = format!("did:seneca:{}",account_pair.public().into_account());
		
		let subject = Subject{
			id: b"did:seneca:5GFEtniprMeFuh8HcoVrWxz4aQtv6T5V9bkENSnfPYhY4p8H".to_vec(),
			claim: vec![claim.clone()],
		};
		let credential_holder = b"did:seneca:5GFEtniprMeFuh8HcoVrWxz4aQtv6T5V9bkENSnfPYhY4p8H".to_vec();
		// Encode and sign the schema message.
		let schema = 123456u32;
		let nonce = 2u64;
		let issuance_date =  Some(Timestamp::now());
		let credential = VerifiableCredential{
				context: context.clone(),
				schema: schema.clone(),
				issuer: account_id.clone().into(),
				issuance_date: issuance_date.clone(),
				expiration_date: Some(1702479816u64),
				subject: subject.clone(),
				credential_holder: credential_holder.clone(),
				nonce: nonce.clone(),
		};
		let updated_credential = VerifiableCredential{
			context: context.clone(),
			schema: schema.clone(),
			issuer: account_id.clone().into(),
			issuance_date: issuance_date.clone(),
			expiration_date: Some(1702379816u64),
			subject: subject.clone(),
			credential_holder: credential_holder.clone(),
			nonce: nonce.clone(),
	};
		let data_sig = account_pair.sign(&credential.encode());
		let updated_sig = account_pair.sign(&updated_credential.encode());
		let schema_id = 0u32;
		// Dispatch a signed create schema extrinsic.
		assert_ok!(SchemaRegistry::create_credential(RawOrigin::Signed(signer).into(), schema_id, context.clone(), schema.clone(), 
													account_id.clone().into(), issuance_date.clone(), Some(1702479816u64),subject.clone(), credential_holder.clone(),data_sig.clone(),
													nonce.clone()));
		assert_ok!(SchemaRegistry::update_credential(RawOrigin::Signed(signer).into(), schema_id.clone(),(updated_sig.clone(), credential.clone())));
		assert_eq!(SchemaRegistry::credential_registry(schema_id.clone()), Some((updated_sig, credential)));

	})
}

#[test]
fn it_works_for_delete_schema() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let name = b"Alice Data".to_vec();
		let expiration_date = Timestamp::now();
		let mandatory_fields = Attribute{
			name: b"name".to_vec(),
			attribute_type: AttributeType::Hex,
		};
		let issuance_req = IssuanceRequirement{
			name: b"issuance_req".to_vec(),
			insuance_type: IssuanceType::Text,
		};
		let claim = Claim{
			property: b"property".to_vec(),
			value: b"value".to_vec(),
			schemaid: None,
			claim_type: ClaimType::CredentialClaim,
			issuance_requirement: Some(vec![issuance_req.clone()]),
		};
		let account_pair = account_pair("Alice");
		let signer = account_pair.public();
		let account_id = format!("did:seneca:{}",account_pair.public().into_account());
		let nonce = 2u64;
		let creation_date = Timestamp::now();
		let schema = VerifiableCredentialSchema {
			name: name.clone(),
			creator: account_id.clone().into(),
			public: false,
			creation_date: creation_date.clone(),
			expiration_date: Some(expiration_date),
			mandatory_fields: vec![mandatory_fields.clone()],
			issuer_claims: vec![claim.clone()],
			subject_claims: vec![claim.clone()],
			credential_claims: vec![claim.clone()],
			metadata: b"metadata".to_vec(),
			nonce,
		};
		let data_sig = account_pair.sign(&schema.encode());
		let schema_id = 0u32;
		// Dispatch a signed create schema extrinsic.
		assert_ok!(SchemaRegistry::create_schema(RawOrigin::Signed(signer).into(), schema_id, name, account_id.clone().into(), false, 
												vec![mandatory_fields], creation_date.clone(), Some(expiration_date), vec![claim.clone()], 
												vec![claim.clone()], vec![claim.clone()], b"metadata".to_vec(), data_sig.clone(), nonce));
		// Dispatch a signed extrinsic.
		assert_ok!(SchemaRegistry::delete_schema(RawOrigin::Signed(signer).into(), schema_id.clone()));
		// Read pallet storage and assert an expected result.
		assert_eq!(SchemaRegistry::schema_registry(schema_id.clone()), None);
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
			schemaid: None,
			claim_type: ClaimType::CredentialClaim,
			issuance_requirement: None,
		};
		let account_pair = account_pair("Alice");
		let signer = account_pair.public();
		let account_id = format!("did:seneca:{}",account_pair.public().into_account());
		
		let subject = Subject{
			id: b"did:seneca:5GFEtniprMeFuh8HcoVrWxz4aQtv6T5V9bkENSnfPYhY4p8H".to_vec(),
			claim: vec![claim.clone()],
		};
		let nonce = 2u64;
		let credential_holder = b"did:seneca:5GFEtniprMeFuh8HcoVrWxz4aQtv6T5V9bkENSnfPYhY4p8H".to_vec();
		let issuance_date = Some(Timestamp::now());
		// Encode and sign the schema message.
		let schema = 123456u32;
		let credential = VerifiableCredential{
			context: context.clone(),
			schema: schema.clone(),
			issuer: account_id.clone().into(),
			issuance_date: issuance_date.clone(),
			expiration_date: Some(1702379816u64),
			subject: subject.clone(),
			credential_holder: credential_holder.clone(),
			nonce: nonce.clone(),
	};
	let schema_id = 0u32;
	let data_sig = account_pair.sign(&credential.encode());
  
	// Dispatch a signed create schema extrinsic.
	assert_ok!(SchemaRegistry::create_credential(RawOrigin::Signed(signer).into(), schema_id,context.clone(), schema, 
												account_id.clone().into(), issuance_date.clone(), Some(1702379816u64),subject.clone(), credential_holder.clone(),data_sig.clone(), 
												nonce));
	// Dispatch a signed extrinsic.
	assert_ok!(SchemaRegistry::delete_credential(RawOrigin::Signed(signer).into(), schema_id.clone()));
	// Read pallet storage and assert an expected result.
	assert_eq!(SchemaRegistry::credential_registry(schema_id.clone()), None);
	});
}