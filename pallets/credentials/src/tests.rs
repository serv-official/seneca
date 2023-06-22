use crate::mock::*;
use crate::types::*;
use codec::Encode;
use frame_support::assert_ok;
use frame_system::RawOrigin;
use pallet_schemas::types::{
	Attribute, AttributeType, Claim, ClaimType, VerifiableCredentialSchema,
};
use sp_core::Pair;
use sp_runtime::traits::IdentifyAccount;

#[test]
fn it_works_for_create_credential() {
	new_test_ext().execute_with(|| {
		let account_pair = account_pair("Alice");
		let signer = account_pair.public();
		let account_id = format!("did:seneca:{}", account_pair.public().to_string());
		let credential = VerifiableCredential {
			context: b"Credential context".to_vec(),
			schema: 123456u32,
			issuer: account_id.clone().into(),
			issuance_date: Some(Timestamp::now()),
			expiration_date: Some(1702379816u64),
			subject: Subject {
				id: b"did:seneca:5GFEtniprMeFuh8HcoVrWxz4aQtv6T5V9bkENSnfPYhY4p8H".to_vec(),
				claim: vec![Claim {
					property: b"property".to_vec(),
					value: b"value".to_vec(),
					schemaid: None,
					claim_type: ClaimType::SubjectClaim,
					issuance_requirement: None,
				}],
			},
			credential_holder: b"did:seneca:5GFEtniprMeFuh8HcoVrWxz4aQtv6T5V9bkENSnfPYhY4p8H"
				.to_vec(),
			nonce: 2u64,
		};
		let vf_schema = VerifiableCredentialSchema {
			name: b"name".to_vec(),
			creator: account_id.clone().into(),
			public: false,
			creation_date: Timestamp::now(),
			expiration_date: credential.expiration_date,
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
		let data_sig = account_pair.sign(&credential.encode());
		let binding = vf_schema.encode();
		let vc_bytes = binding.as_slice();
		let schema_data_sig = account_pair.sign(&vc_bytes);
		assert_ok!(SchemaRegistry::create_schema(
			RawOrigin::Signed(signer).into(),
			credential.schema,
			b"name".to_vec(),
			vf_schema.creator,
			false,
			vf_schema.mandatory_fields,
			Timestamp::now(),
			credential.expiration_date,
			vf_schema.issuer_claims,
			vf_schema.subject_claims,
			vf_schema.credential_claims,
			b"metadata".to_vec(),
			schema_data_sig,
			credential.nonce
		));
		// Dispatch a signed create schema extrinsic.
		assert_ok!(CredentialRegistry::create_credential(
			RawOrigin::Signed(signer).into(),
			credential.schema,
			credential.context,
			credential.schema,
			credential.issuer,
			credential.issuance_date,
			Some(1702379816u64),
			credential.subject,
			credential.credential_holder,
			data_sig,
			credential.nonce
		));
	});
}

#[test]
fn it_works_for_update_credential() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let account_pair = account_pair("Alice");
		let signer = account_pair.public();
		let account_id = format!("did:seneca:{}", account_pair.public().into_account());
		let credential = VerifiableCredential {
			context: b"Credential context".to_vec(),
			schema: 123456u32,
			issuer: account_id.clone().into(),
			issuance_date: Some(Timestamp::now()),
			expiration_date: Some(1702379816u64),
			subject: Subject {
				id: b"did:seneca:5GFEtniprMeFuh8HcoVrWxz4aQtv6T5V9bkENSnfPYhY4p8H".to_vec(),
				claim: vec![Claim {
					property: b"property".to_vec(),
					value: b"value".to_vec(),
					schemaid: None,
					claim_type: ClaimType::SubjectClaim,
					issuance_requirement: None,
				}],
			},
			credential_holder: b"did:seneca:5GFEtniprMeFuh8HcoVrWxz4aQtv6T5V9bkENSnfPYhY4p8H"
				.to_vec(),
			nonce: 2u64,
		};
		let updated_credential = VerifiableCredential {
			context: b"Credential context".to_vec(),
			schema: 123456u32,
			issuer: account_id.clone().into(),
			issuance_date: Some(Timestamp::now()),
			expiration_date: Some(1702379816u64),
			subject: Subject {
				id: b"did:seneca:5GFEtniprMeFuh8HcoVrWxz4aQtv6T5V9bkENSnfPYhY4p8H".to_vec(),
				claim: vec![Claim {
					property: b"property".to_vec(),
					value: b"value".to_vec(),
					schemaid: None,
					claim_type: ClaimType::SubjectClaim,
					issuance_requirement: None,
				}],
			},
			credential_holder: b"did:seneca:5GFEtniprMeFuh8HcoVrWxz4aQtv6T5V9bkENSnfPYhY4p8H"
				.to_vec(),
			nonce: 2u64,
		};
		let vf_schema = VerifiableCredentialSchema {
			name: b"name".to_vec(),
			creator: account_id.clone().into(),
			public: false,
			creation_date: Timestamp::now(),
			expiration_date: credential.expiration_date,
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
		let data_sig = account_pair.sign(&credential.encode());
		let updated_sig = account_pair.sign(&updated_credential.encode());
		let binding = vf_schema.encode();
		let vc_bytes = binding.as_slice();
		let schema_data_sig = account_pair.sign(&vc_bytes);
		// Dispatch a signed create schema extrinsic.
		assert_ok!(SchemaRegistry::create_schema(
			RawOrigin::Signed(signer).into(),
			credential.schema,
			b"name".to_vec(),
			vf_schema.creator,
			false,
			vf_schema.mandatory_fields,
			Timestamp::now(),
			vf_schema.expiration_date,
			vf_schema.issuer_claims,
			vf_schema.subject_claims,
			vf_schema.credential_claims,
			b"metadata".to_vec(),
			schema_data_sig,
			credential.nonce
		));
		assert_ok!(CredentialRegistry::create_credential(
			RawOrigin::Signed(signer).into(),
			credential.schema,
			credential.context.clone(),
			credential.schema,
			credential.issuer,
			credential.issuance_date.clone(),
			Some(1702379816u64),
			credential.subject.clone(),
			credential.credential_holder.clone(),
			data_sig.clone(),
			credential.nonce
		));
		assert_ok!(CredentialRegistry::update_credential(
			RawOrigin::Signed(signer).into(),
			credential.schema,
			(updated_sig.clone(), updated_credential.clone())
		));
		assert_eq!(
			CredentialRegistry::credential_registry(credential.schema),
			Some((updated_sig, updated_credential.clone()))
		);
	})
}

#[test]
fn it_works_for_delete_credential() {
	new_test_ext().execute_with(|| {
		let account_pair = account_pair("Alice");
		let signer = account_pair.public();
		let account_id = format!("did:seneca:{}", account_pair.public().into_account());
		let credential = VerifiableCredential {
			context: b"Credential context".to_vec(),
			schema: 123456u32,
			issuer: account_id.clone().into(),
			issuance_date: Some(Timestamp::now()),
			expiration_date: Some(1702379816u64),
			subject: Subject {
				id: b"did:seneca:5GFEtniprMeFuh8HcoVrWxz4aQtv6T5V9bkENSnfPYhY4p8H".to_vec(),
				claim: vec![Claim {
					property: b"property".to_vec(),
					value: b"value".to_vec(),
					schemaid: None,
					claim_type: ClaimType::SubjectClaim,
					issuance_requirement: None,
				}],
			},
			credential_holder: b"did:seneca:5GFEtniprMeFuh8HcoVrWxz4aQtv6T5V9bkENSnfPYhY4p8H"
				.to_vec(),
			nonce: 2u64,
		};
		let vf_schema = VerifiableCredentialSchema {
			name: b"name".to_vec(),
			creator: account_id.clone().into(),
			public: false,
			creation_date: Timestamp::now(),
			expiration_date: credential.expiration_date,
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
		let data_sig = account_pair.sign(&credential.encode());
		let binding = vf_schema.encode();
		let vc_bytes = binding.as_slice();
		let schema_data_sig = account_pair.sign(&vc_bytes);
		// Dispatch a signed create schema extrinsic.
		assert_ok!(SchemaRegistry::create_schema(
			RawOrigin::Signed(signer).into(),
			credential.schema,
			b"name".to_vec(),
			vf_schema.creator,
			false,
			vf_schema.mandatory_fields,
			Timestamp::now(),
			vf_schema.expiration_date,
			vf_schema.issuer_claims,
			vf_schema.subject_claims,
			vf_schema.credential_claims,
			b"metadata".to_vec(),
			schema_data_sig,
			credential.nonce
		));

		assert_ok!(CredentialRegistry::create_credential(
			RawOrigin::Signed(signer).into(),
			credential.schema,
			credential.context.clone(),
			credential.schema,
			credential.issuer,
			credential.issuance_date.clone(),
			Some(1702379816u64),
			credential.subject.clone(),
			credential.credential_holder.clone(),
			data_sig.clone(),
			credential.nonce
		));

		assert_ok!(CredentialRegistry::delete_credential(
			RawOrigin::Signed(signer).into(),
			credential.schema.clone()
		));

		assert_eq!(CredentialRegistry::credential_registry(credential.schema.clone()), None);
	});
}
