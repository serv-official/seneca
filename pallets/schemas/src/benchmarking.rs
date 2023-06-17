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
use sp_application_crypto::sr25519::Public;
use sp_application_crypto::RuntimePublic;
use sp_runtime::traits::IdentifyAccount;

benchmarks! {

	create_schema{
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		let name = b"Alice Data".to_vec();
		let creator = b"did:seneca:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let expiration_date: T::Moment = Default::default();
		let nonce = 2u64;
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

		//create random schema_id
		let schema_id: T::SchemaId = Default::default();
		let keypair = Public::generate_pair(sp_core::testing::SR25519, None);
		let pub_key: T::Public = keypair.into();
		let account_id = format!("did:seneca:{:#?}",pub_key.into_account());
		let creation_date: T::Moment = Default::default();
		let schema: VerifiableCredentialSchema<T::Moment> = VerifiableCredentialSchema {
			name: name.clone(),
			creator: account_id.clone().encode(),
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

		// sign the schema in benchmarks
		let sig: T::Signature = keypair.sign(sp_core::testing::SR25519, &schema.encode()).unwrap().into();

		// Encode and sign the schema message.
	}:  _(RawOrigin::Signed(caller), schema_id.clone(), name.clone(), account_id.clone().encode(), false,
			vec![mandatory_fields.clone()], creation_date, Some(expiration_date), vec![claim.clone()],
			vec![claim.clone()], vec![claim.clone()], b"metadata".to_vec(), sig.clone(),nonce )
	verify {
		//assert that the schema stored is different from the one created since the nonce is different.
		assert_eq!(SchemaStore::<T>::get(schema_id), Some((sig, schema)));
	}
	update_schema{
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		// Dispatch a signed extrinsic.
		let name = b"Alice Data".to_vec();
		let expiration_date: T::Moment = Default::default();
		let mandatory_fields = Attribute{
			name: b"name".to_vec(),
			attribute_type: AttributeType::Hex,
		};
		let issuance_req = IssuanceRequirement{
			name: b"issuance_req".to_vec(),
			insuance_type: IssuanceType::Text,
		};
		let nonce = 2u64;
		let claim = Claim{
			property: b"property".to_vec(),
			value: b"value".to_vec(),
			schemaid: None,
			claim_type: ClaimType::CredentialClaim,
			issuance_requirement: Some(vec![issuance_req.clone()]),
		};
		//create random schema_id
		let schema_id: T::SchemaId = Default::default();
		let keypair = Public::generate_pair(sp_core::testing::SR25519, None);
		let pub_key: T::Public = keypair.into();
		let account_id = format!("did:seneca:{:#?}",pub_key.into_account());
		let creation_date: T::Moment =  Default::default();
		let schema: VerifiableCredentialSchema<T::Moment> = VerifiableCredentialSchema {
			name: name.clone(),
			creator: account_id.clone().encode(),
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
			creator: account_id.clone().encode(),
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

		// sign the schema in benchmarks
		let sig: T::Signature = keypair.sign(sp_core::testing::SR25519, &schema.encode()).unwrap().into();

		assert_ok!(SchemaRegistry::<T>::create_schema(RawOrigin::Signed(caller.clone()).into(), schema_id.clone(), name, account_id.encode(), false,
									vec![mandatory_fields], creation_date.clone(), Some(expiration_date), vec![claim.clone()],
									vec![claim.clone()], vec![claim.clone()], b"metadata".to_vec(), sig.clone(), nonce));
	}:  _(RawOrigin::Signed(caller), schema_id.clone(), (sig.clone(), updated_schema.clone()))
	verify {
		assert_eq!(SchemaStore::<T>::get(schema_id), Some((sig, updated_schema)));
	}

	delete_schema{
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		// Dispatch a signed extrinsic.
		let name = b"Alice Data".to_vec();
		let creator = b"did:seneca:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let expiration_date: T::Moment = Default::default();
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
		let nonce = 2u64;

		//create random schema_id
		let schema_id: T::SchemaId = Default::default();

		// sign the schema in benchmarks
		let keypair = Public::generate_pair(sp_core::testing::SR25519, None);
		let pub_key: T::Public = keypair.into();
		let account_id = format!("did:seneca:{:#?}",pub_key.into_account());
		let creation_date: T::Moment  = Default::default();
		let schema: VerifiableCredentialSchema<T::Moment> = VerifiableCredentialSchema {
			name: name.clone(),
			creator: account_id.clone().encode(),
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

		let sig: T::Signature = keypair.sign(sp_core::testing::SR25519, &schema.encode()).unwrap().into();

		assert_ok!(SchemaRegistry::<T>::create_schema(RawOrigin::Signed(caller.clone()).into(), schema_id.clone(), name, account_id.encode(), false,
									vec![mandatory_fields], creation_date, Some(expiration_date), vec![claim.clone()],
									vec![claim.clone()], vec![claim.clone()], b"metadata".to_vec(), sig, nonce));
	}:  _(RawOrigin::Signed(caller), schema_id.clone())
	verify {
		assert_eq!(SchemaStore::<T>::get(schema_id), None);
	}
	impl_benchmark_test_suite!(SchemaRegistry, crate::mock::new_test_ext(), crate::mock::Test);
}
