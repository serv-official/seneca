//! Benchmarking setup for pallet-template
use super::*;
use crate::types::*;
use crate::Pallet as CredentialRegistry;
use pallet_schemas::Pallet as SchemaRegistry;
use codec::Encode;
use frame_benchmarking::benchmarks;
use frame_support::assert_ok;
use frame_system::RawOrigin;
use scale_info::prelude::format;
use scale_info::prelude::vec;
use pallet_schemas::types::{
	Attribute, AttributeType, Claim, ClaimType, VerifiableCredentialSchema,
};
use sp_application_crypto::RuntimePublic;
use sp_application_crypto::sr25519::Public;


benchmarks! {
    where_clause {
        where T::AccountId: From<sp_core::sr25519::Public>,
			  T::Signature : From<sp_core::sr25519::Signature>
    }
	create_credential{
		let s in 0 .. 100;
		//create random schema_id
		let schema_id = 123u32;
		let credential_id: T::CredentialId = Default::default();
		let public = Public::generate_pair(sp_core::testing::SR25519, None);
		let caller: T::AccountId = public.into();
		let account_id = format!("did:seneca:{:#?}", caller.clone());
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
		let data_sig = public.sign(sp_core::testing::SR25519, &credential.encode()).unwrap();
		let binding = vf_schema.encode();
		let vc_bytes = binding.as_slice();
		let schema_data_sig = public.sign(sp_core::testing::SR25519, &vc_bytes.encode()).unwrap();
		
		assert_ok!(SchemaRegistry::<T>::create_schema(
			RawOrigin::Signed(caller.clone()).into(),
			credential.schema.into(),
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
			schema_data_sig.into(),
			vf_schema.nonce
		));
		// Encode and sign the schema message.
	}:  _(
		RawOrigin::Signed(caller), 
		credential_id.clone(),
		credential.clone().context,
		credential.clone().schema,
		credential.clone().issuer,
		credential.clone().issuance_date,
		credential.clone().expiration_date,
		credential.clone().subject,
		credential.clone().credential_holder,
		data_sig.clone().into(),
		credential.clone().nonce)
	verify {
		//assert that the credential stored is different from the one created since the nonce is different.
		assert_eq!(CredentialStore::<T>::get(credential_id), Some((data_sig.clone().into(), credential)));
	}
	update_credential{
		let s in 0 .. 100;
		//create random schema_id
		let schema_id = 123u32;
		let credential_id: T::CredentialId = Default::default();
		let public = Public::generate_pair(sp_core::testing::SR25519, None);
		let caller: T::AccountId = public.into();
		let account_id = format!("did:seneca:{:#?}", caller.clone());
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
		let sig = public.sign(sp_core::testing::SR25519, &credential.encode()).unwrap();
		let binding = vf_schema.encode();
		let vc_bytes = binding.as_slice();
		let schema_data_sig = public.sign(sp_core::testing::SR25519, &vc_bytes.encode()).unwrap();

		assert_ok!(SchemaRegistry::<T>::create_schema(
			RawOrigin::Signed(caller.clone()).into(),
			credential.schema.into(),
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
			schema_data_sig.into(),
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

		//create random schema_id
		let schema_id = 123u32;
		let credential_id: T::CredentialId = Default::default();
		let public = Public::generate_pair(sp_core::testing::SR25519, None);
		let caller: T::AccountId = public.into();
		let account_id = format!("did:seneca:{:#?}", caller.clone());
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
		let sig = public.sign(sp_core::testing::SR25519, &credential.encode()).unwrap();
		let binding = vf_schema.encode();
		let vc_bytes = binding.as_slice();
		let schema_data_sig = public.sign(sp_core::testing::SR25519, &vc_bytes.encode()).unwrap();

		assert_ok!(SchemaRegistry::<T>::create_schema(
			RawOrigin::Signed(caller.clone()).into(),
			credential.schema.into(),
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
			schema_data_sig.into(),
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
	}:  _(RawOrigin::Signed(caller), credential_id.clone())
	verify {
		assert_eq!(CredentialStore::<T>::get(credential_id), None);
	}
	impl_benchmark_test_suite!(CredentialRegistry, crate::mock::new_test_ext(), crate::mock::Test);
}
