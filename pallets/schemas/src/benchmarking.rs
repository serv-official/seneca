//! Benchmarking setup for pallet-template
use super::*;
use crate::types::*;
#[allow(unused)]
use crate::Pallet as SchemaRegistry;
use codec::Encode;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_support::assert_ok;
use frame_system::RawOrigin;
use scale_info::prelude::format;
use scale_info::prelude::vec;
use sp_runtime::traits::IdentifyAccount;
use sp_application_crypto::RuntimePublic;
use sp_application_crypto::sr25519::Public;

benchmarks! {

	create_schema{
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		//create random schema_id
		let schema_id: T::SchemaId = Default::default();
		let keypair = Public::generate_pair(sp_core::testing::SR25519, None);
		let account_id = format!("did:seneca:{}", keypair.into_account());
		let schema: VerifiableCredentialSchema<T::Moment> = VerifiableCredentialSchema {
			name: b"Alice Data".to_vec(),
			creator: account_id.into(),
			public: false,
			creation_date: Default::default(),
			expiration_date: Some(Default::default()),
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
		let sig = keypair.sign(sp_core::testing::SR25519, &schema.encode()).unwrap();
		// Encode and sign the schema message.
	}:  _(RawOrigin::Signed(caller), 
			schema_id.clone(),
			schema.clone().name,
			schema.clone().creator,
			false,
			schema.clone().mandatory_fields,
			schema.clone().creation_date,
			schema.clone().expiration_date,
			schema.clone().issuer_claims,
			schema.clone().subject_claims,
			schema.clone().credential_claims,
			schema.clone().metadata,
			sig.clone().into(),
			schema.clone().nonce)
	verify {
		//assert that the schema stored is different from the one created since the nonce is different.
		assert_eq!(SchemaStore::<T>::get(schema_id), Some((sig.into(), schema)));
	}
	update_schema{
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		let schema_id: T::SchemaId = Default::default();
		let keypair = Public::generate_pair(sp_core::testing::SR25519, None);
		let account_id = format!("did:seneca:{}", keypair.into_account());
		let schema: VerifiableCredentialSchema<T::Moment> = VerifiableCredentialSchema {
			name: b"Alice Data".to_vec(),
			creator: account_id.clone().into(),
			public: false,
			creation_date: Default::default(),
			expiration_date: Some(Default::default()),
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
			creator: account_id.into(),
			public: false,
			creation_date: Default::default(),
			expiration_date: Some(Default::default()),
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

		// sign the schema in benchmarks
		let sig = keypair.sign(sp_core::testing::SR25519, &schema.encode()).unwrap();

		assert_ok!(SchemaRegistry::<T>::create_schema(
			RawOrigin::Signed(caller.clone()).into(), 
			schema_id.clone(),
			schema.name,
			schema.creator,
			false,
			schema.mandatory_fields,
			schema.creation_date,
			schema.expiration_date,
			schema.issuer_claims,
			schema.subject_claims,
			schema.credential_claims,
			schema.metadata,
			sig.clone().into(),
			schema.nonce
		));
	}:  _(RawOrigin::Signed(caller), schema_id.clone(), (sig.clone().into(), updated_schema.clone()))
	verify {
		assert_eq!(SchemaStore::<T>::get(schema_id.clone()), Some((sig.into(), updated_schema)));
	}

	delete_schema{
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		// Dispatch a signed extrinsic.
		let schema_id: T::SchemaId = Default::default();
		let keypair = Public::generate_pair(sp_core::testing::SR25519, None);
		let account_id = format!("did:seneca:{}", keypair.into_account());
		let schema: VerifiableCredentialSchema<T::Moment> = VerifiableCredentialSchema {
			name: b"Alice Data".to_vec(),
			creator: account_id.into(),
			public: false,
			creation_date: Default::default(),
			expiration_date: Some(Default::default()),
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

		let sig = keypair.sign(sp_core::testing::SR25519, &schema.encode()).unwrap();

		assert_ok!(SchemaRegistry::<T>::create_schema(
			RawOrigin::Signed(caller.clone()).into(), 
			schema_id.clone(),
			schema.name,
			schema.creator,
			false,
			schema.mandatory_fields,
			schema.creation_date,
			schema.expiration_date,
			schema.issuer_claims,
			schema.subject_claims,
			schema.credential_claims,
			schema.metadata,
			sig.clone().into(),
			schema.nonce
		));
	}:  _(RawOrigin::Signed(caller), schema_id.clone())
	verify {
		assert_eq!(SchemaStore::<T>::get(schema_id), None);
	}
	impl_benchmark_test_suite!(SchemaRegistry, crate::mock::new_test_ext(), crate::mock::Test);
}
