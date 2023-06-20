//! Benchmarking setup for pallet-template
use super::*;
use crate::types::*;
use crate::Pallet as CredentialRegistry;
use pallet_schemas::Pallet as SchemaRegistry;
use codec::Encode;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_support::assert_ok;
use frame_system::RawOrigin;
use scale_info::prelude::format;
use scale_info::prelude::vec;
use sp_application_crypto::sr25519;
use sp_core::Pair;
use pallet_schemas::types::{
	Attribute, AttributeType, Claim, ClaimType, VerifiableCredentialSchema,
};
use sp_runtime::traits::IdentifyAccount;

benchmarks! {

	create_credential{
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		//create random schema_id
		let schema_id = 123u32;
		let credential_id: T::CredentialId = Default::default();
		let keypair = sr25519::Pair::generate();
		let pair = keypair.0;
		let account_id = format!("did:seneca:{}", pair.public().into_account());
		let creation_date: T::Moment = Default::default();
		let credential: VerifiableCredential<T::Moment> = VerifiableCredential {
			context: b"Credential context".to_vec(),
			schema: schema_id,
			issuer: account_id.clone().into(),
			issuance_date: Some(Default::default()),
			expiration_date: Some(Default::default()),
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
		let vf_schema: VerifiableCredentialSchema<T::Moment> = VerifiableCredentialSchema {
			name: b"name".to_vec(),
			creator: account_id.clone().into(),
			public: false,
			creation_date: Default::default(),
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
		// sign the schema in benchmarks
		let data_sig = pair.sign(&credential.encode());
		let binding = vf_schema.encode();
		let vc_bytes = binding.as_slice();
		let schema_data_sig = pair.sign(&vc_bytes);

		assert_ok!(SchemaRegistry::create_schema(
			RawOrigin::Signed(caller.clone()).into(),
			credential.schema,
			b"name".to_vec(),
			vf_schema.creator,
			false,
			vf_schema.mandatory_fields,
			vf_schema.creation_date,
			vf_schema.expiration_date,
			vf_schema.issuer_claims,
			vf_schema.subject_claims,
			vf_schema.credential_claims,
			b"metadata".to_vec(),
			schema_data_sig,
			vf_schema.nonce
		));
		// Encode and sign the schema message.
	}:  _(
		RawOrigin::Signed(caller), 
		credential_id.clone(),
		credential.context,
		credential.schema,
		credential.issuer,
		credential.issuance_date,
		credential.expiration_date,
		credential.subject,
		credential.credential_holder,
		data_sig.into(),
		credential.nonce)
	verify {
		//assert that the credential stored is different from the one created since the nonce is different.
		assert_eq!(CredentialStore::<T>::get(credential_id), Some((data_sig.into(), credential)));
	}
	update_credential{
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();	
		//create random schema_id
		let schema_id = 123u32;
		let credential_id: T::CredentialId = Default::default();
		let keypair = sr25519::Pair::generate();
		let pair = keypair.0;
		let account_id = format!("did:seneca:{}", pair.public().into_account());
		let creation_date: T::Moment =  Default::default();
		let credential: VerifiableCredential<T::Moment> = VerifiableCredential {
			context: b"Credential context".to_vec(),
			schema: schema_id,
			issuer: account_id.clone().into(),
			issuance_date: Some(Default::default()),
			expiration_date: Some(Default::default()),
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
		let updated_credential:VerifiableCredential<T::Moment> = VerifiableCredential {
			context: b"Credential context".to_vec(),
			schema: schema_id,
			issuer: account_id.clone().into(),
			issuance_date: Some(Default::default()),
			expiration_date: Some(Default::default()),
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
			nonce: 3u64,
		};

		let vf_schema: VerifiableCredentialSchema<T::Moment> = VerifiableCredentialSchema {
			name: b"name".to_vec(),
			creator: account_id.clone().into(),
			public: false,
			creation_date: Default::default(),
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

		// sign the schema in benchmarks
		let sig = pair.sign(&credential.encode());
		let binding = vf_schema.encode();
		let vc_bytes = binding.as_slice();
		let schema_data_sig = pair.sign(&vc_bytes);

		assert_ok!(SchemaRegistry::create_schema(
			RawOrigin::Signed(caller.clone()).into(),
			credential.schema,
			b"name".to_vec(),
			vf_schema.creator,
			false,
			vf_schema.mandatory_fields,
			vf_schema.creation_date,
			vf_schema.expiration_date,
			vf_schema.issuer_claims,
			vf_schema.subject_claims,
			vf_schema.credential_claims,
			b"metadata".to_vec(),
			schema_data_sig,
			vf_schema.nonce
		));

		assert_ok!(CredentialRegistry::<T>::create_credential(
			RawOrigin::Signed(caller.clone()).into(), 
			credential_id.clone(),
			credential.context,
			credential.schema,
			credential.issuer,
			credential.issuance_date,
			credential.expiration_date,
			credential.subject,
			credential.credential_holder,
			sig.clone().into(),
			credential.nonce
		));
	}:  _(RawOrigin::Signed(caller), credential_id.clone().into(), (sig.clone().into(), updated_credential.clone()))
	verify {
		assert_eq!(CredentialStore::<T>::get(credential_id), Some((sig.into(), updated_credential)));
	}

	delete_credential{
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();

		//create random schema_id
		let schema_id = 123u32;
		let credential_id: T::CredentialId = Default::default();

		// sign the schema in benchmarks
		let keypair = sr25519::Pair::generate();
		let pair = keypair.0;
		let account_id = format!("did:seneca:{}", pair.public().into_account());
		let credential: VerifiableCredential<T::Moment> = VerifiableCredential {
			context: b"Credential context".to_vec(),
			schema: schema_id,
			issuer: account_id.clone().into(),
			issuance_date: Some(Default::default()),
			expiration_date: Some(Default::default()),
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

		// sign the schema in benchmarks
		let sig = pair.sign(&credential.encode());

		assert_ok!(CredentialRegistry::<T>::create_credential(
			RawOrigin::Signed(caller.clone()).into(), 
			credential_id.clone(),
			credential.context,
			credential.schema,
			credential.issuer,
			credential.issuance_date,
			credential.expiration_date,
			credential.subject,
			credential.credential_holder,
			sig.clone().into(),
			credential.nonce
		));
	}:  _(RawOrigin::Signed(caller), credential_id.clone())
	verify {
		assert_eq!(CredentialStore::<T>::get(credential_id), None);
	}
	impl_benchmark_test_suite!(CredentialRegistry, crate::mock::new_test_ext(), crate::mock::Test);
}
