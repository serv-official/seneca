use crate::mock::*;
use crate::types::*;
use codec::Encode;
use frame_support::assert_ok;
use frame_system::RawOrigin;
use sp_core::Pair;
use sp_runtime::traits::IdentifyAccount;

#[test]
fn it_works_for_create_schema() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let account_pair = account_pair("Alice");
		let signer = account_pair.public();
		let account_id = format!("did:seneca:{}", account_pair.public().into_account());
		// Encode and sign the schema struct.
		let schema = VerifiableCredentialSchema {
			name: b"Alice Data".to_vec(),
			creator: account_id.clone().into(),
			public: false,
			creation_date: Timestamp::now(),
			expiration_date: Some(1702379816u64),
			mandatory_fields: vec![Attribute {
				name: b"name".to_vec(),
				attribute_type: AttributeType::Hex,
			}],
			issuer_claims: vec![Claim {
				property: b"property".to_vec(),
				value: b"value".to_vec(),
				schemaid: None,
				claim_type: ClaimType::IssuerClaim,
				issuance_requirement: None,
			}],
			subject_claims: vec![Claim {
				property: b"property".to_vec(),
				value: b"value".to_vec(),
				schemaid: None,
				claim_type: ClaimType::SubjectClaim,
				issuance_requirement: None,
			}],
			credential_claims: vec![Claim {
				property: b"property".to_vec(),
				value: b"value".to_vec(),
				schemaid: None,
				claim_type: ClaimType::CredentialClaim,
				issuance_requirement: None,
			}],
			metadata: b"metadata".to_vec(),
			nonce: 2u64,
		};
		let schema_id = 0u32;
		//dbg!("Schema: {:?}", schema);
		let binding = schema.encode();
		let vc_bytes = binding.as_slice();
		let data_sig = account_pair.sign(&vc_bytes);
		// Dispatch a signed create schema extrinsic.
		assert_ok!(SchemaRegistry::create_schema(
			RawOrigin::Signed(signer).into(),
			schema_id,
			schema.name,
			schema.creator,
			false,
			schema.mandatory_fields,
			schema.creation_date,
			schema.expiration_date,
			schema.issuer_claims,
			schema.subject_claims,
			schema.credential_claims,
			b"metadata".to_vec(),
			data_sig,
			schema.nonce
		));
	});
}

#[test]
fn it_works_for_update_schema() {
	new_test_ext().execute_with(|| {
		let account_pair = account_pair("Alice");
		let signer = account_pair.public();
		let account_id = format!("did:seneca:{}", account_pair.public().into_account());

		let schema = VerifiableCredentialSchema {
			name: b"Alice Data".to_vec(),
			creator: account_id.clone().into(),
			public: false,
			creation_date: Timestamp::now(),
			expiration_date: Some(1702379816u64),
			mandatory_fields: vec![Attribute {
				name: b"name".to_vec(),
				attribute_type: AttributeType::Hex,
			}],
			issuer_claims: vec![Claim {
				property: b"property".to_vec(),
				value: b"value".to_vec(),
				schemaid: None,
				claim_type: ClaimType::IssuerClaim,
				issuance_requirement: None,
			}],
			subject_claims: vec![Claim {
				property: b"property".to_vec(),
				value: b"value".to_vec(),
				schemaid: None,
				claim_type: ClaimType::SubjectClaim,
				issuance_requirement: None,
			}],
			credential_claims: vec![Claim {
				property: b"property".to_vec(),
				value: b"value".to_vec(),
				schemaid: None,
				claim_type: ClaimType::CredentialClaim,
				issuance_requirement: None,
			}],
			metadata: b"metadata".to_vec(),
			nonce: 2u64,
		};
		let updated_schema = VerifiableCredentialSchema {
			name: b"Alice Data".to_vec(),
			creator: account_id.clone().into(),
			public: false,
			creation_date: Timestamp::now(),
			expiration_date: Some(1702379816u64),
			mandatory_fields: vec![Attribute {
				name: b"name".to_vec(),
				attribute_type: AttributeType::Hex,
			}],
			issuer_claims: vec![Claim {
				property: b"property".to_vec(),
				value: b"value".to_vec(),
				schemaid: None,
				claim_type: ClaimType::IssuerClaim,
				issuance_requirement: None,
			}],
			subject_claims: vec![Claim {
				property: b"property".to_vec(),
				value: b"value".to_vec(),
				schemaid: None,
				claim_type: ClaimType::SubjectClaim,
				issuance_requirement: None,
			}],
			credential_claims: vec![Claim {
				property: b"property".to_vec(),
				value: b"value".to_vec(),
				schemaid: None,
				claim_type: ClaimType::CredentialClaim,
				issuance_requirement: None,
			}],
			metadata: b"metadata2".to_vec(),
			nonce: 2u64,
		};
		let data_sig = account_pair.sign(&schema.encode());
		let updated_sig = account_pair.sign(&updated_schema.encode());
		let schema_id = 0u32;
		// Dispatch a signed extrinsic.
		assert_ok!(SchemaRegistry::create_schema(
			RawOrigin::Signed(signer).into(),
			schema_id,
			schema.name,
			schema.creator,
			false,
			schema.mandatory_fields,
			schema.creation_date,
			schema.expiration_date,
			schema.issuer_claims,
			schema.subject_claims,
			schema.credential_claims,
			b"metadata".to_vec(),
			data_sig,
			schema.nonce
		));
		assert_ok!(SchemaRegistry::update_schema(
			RawOrigin::Signed(signer).into(),
			schema_id,
			(updated_sig.clone(), updated_schema.clone())
		));
		assert_eq!(
			SchemaRegistry::schema_registry(schema_id.clone()),
			Some((updated_sig, updated_schema))
		);
	})
}

#[test]
fn it_works_for_delete_schema() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let account_pair = account_pair("Alice");
		let signer = account_pair.public();
		let account_id = format!("did:seneca:{}", account_pair.public().into_account());
		let schema = VerifiableCredentialSchema {
			name: b"Alice Data".to_vec(),
			creator: account_id.clone().into(),
			public: false,
			creation_date: Timestamp::now(),
			expiration_date: Some(1702379816u64),
			mandatory_fields: vec![Attribute {
				name: b"name".to_vec(),
				attribute_type: AttributeType::Hex,
			}],
			issuer_claims: vec![Claim {
				property: b"property".to_vec(),
				value: b"value".to_vec(),
				schemaid: None,
				claim_type: ClaimType::IssuerClaim,
				issuance_requirement: None,
			}],
			subject_claims: vec![Claim {
				property: b"property".to_vec(),
				value: b"value".to_vec(),
				schemaid: None,
				claim_type: ClaimType::SubjectClaim,
				issuance_requirement: None,
			}],
			credential_claims: vec![Claim {
				property: b"property".to_vec(),
				value: b"value".to_vec(),
				schemaid: None,
				claim_type: ClaimType::CredentialClaim,
				issuance_requirement: None,
			}],
			metadata: b"metadata".to_vec(),
			nonce: 2u64,
		};
		let data_sig = account_pair.sign(&schema.encode());
		let schema_id = 0u32;
		// Dispatch a signed create schema extrinsic.
		assert_ok!(SchemaRegistry::create_schema(
			RawOrigin::Signed(signer).into(),
			schema_id,
			schema.name,
			schema.creator,
			false,
			schema.mandatory_fields,
			schema.creation_date,
			schema.expiration_date,
			schema.issuer_claims,
			schema.subject_claims,
			schema.credential_claims,
			b"metadata".to_vec(),
			data_sig,
			schema.nonce
		));
		// Dispatch a signed extrinsic.
		assert_ok!(SchemaRegistry::delete_schema(
			RawOrigin::Signed(signer).into(),
			schema_id.clone()
		));
		// Read pallet storage and assert an expected result.
		assert_eq!(SchemaRegistry::schema_registry(schema_id.clone()), None);
	});
}
