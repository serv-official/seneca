//! Benchmarking setup for pallet-template

use super::*;
#[allow(unused)]
use crate::Pallet as SchemaRegistry;
use sp_core::Encode;
use sp_core::sr25519;
use hex_literal::hex;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use scale_info::prelude::vec;
use frame_support::assert_ok;
use crate::types::*;


benchmarks! {

	create_schema{
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		let name = b"Alice Data".to_vec();
		let creator = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let expiration_date: T::Moment = Default::default();
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
		let insuance_req = InsuanceRequirement{
			name: b"insuance_req".to_vec(),
			insuance_type: InsuanceType::String,
		};
		let schema: VerifiableCredentialSchema< T::Moment> = VerifiableCredentialSchema {
			name: name.clone(),
			creator: creator.clone(),
			public: false,
			creation_date: Default::default(),
			expiration_date: Some(expiration_date),
			mandatory_fields: vec![mandatory_fields.clone()],
			issuer_claims: vec![claim.clone()],
			subject_claims: vec![claim.clone()],
			credential_claims: vec![claim.clone()],
			insuance_req: insuance_req.clone(),
			metadata: b"metadata".to_vec(),
			nonce: 2,
		};
		// sign the schema.
		// must be same type as T::Signature
		let sig: T::Signature = sr25519::Signature::from_slice(&hex!("a6350211fcdf1d7f0c79bf0a9c296de17449ca88a899f0cd19a70b07513fc107b7d34249dba71d4761ceeec2ed6bc1305defeb96418e6869e6b6199ed0de558e")).unwrap().into();
		// Encode and sign the schema message.
	}:  _(RawOrigin::Signed(caller), name.clone(), creator.clone(), false,
			vec![mandatory_fields.clone()], Some(expiration_date), vec![claim.clone()], 
			vec![claim.clone()], vec![claim.clone()], insuance_req.clone(), b"metadata".to_vec(), sig.clone() )
	verify {
		//assert that the schema stored is different from the one created since the nonce is different.
		assert_ne!(SchemaStore::<T>::get(sig.clone()), Some(schema));
	}
	update_schema{
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		// Dispatch a signed extrinsic.
		let name = b"Alice Data".to_vec();
		let creator = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let expiration_date: T::Moment = Default::default();
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
		let insuance_req = InsuanceRequirement{
			name: b"insuance_req".to_vec(),
			insuance_type: InsuanceType::String,
		};
		let schema: VerifiableCredentialSchema<T::Moment> = VerifiableCredentialSchema {
			name: name.clone(),
			creator: creator.clone(),
			public: false,
			creation_date: Default::default(),
			expiration_date: Some(expiration_date),
			mandatory_fields: vec![mandatory_fields.clone()],
			issuer_claims: vec![claim.clone()],
			subject_claims: vec![claim.clone()],
			credential_claims: vec![claim.clone()],
			insuance_req: insuance_req.clone(),
			metadata: b"metadata".to_vec(),
			nonce: 2,
		};
		let sig: T::Signature = sr25519::Signature::from_slice(&hex!("a6350211fcdf1d7f0c79bf0a9c296de17449ca88a899f0cd19a70b07513fc107b7d34249dba71d4761ceeec2ed6bc1305defeb96418e6869e6b6199ed0de558e")).unwrap().into();
		assert_ok!(SchemaRegistry::<T>::create_schema(RawOrigin::Signed(caller.clone()).into(), name, creator, false,
									vec![mandatory_fields], Some(expiration_date), vec![claim.clone()], 
									vec![claim.clone()], vec![claim.clone()],insuance_req.clone(), b"metadata".to_vec(), sig.clone()));
	}:  _(RawOrigin::Signed(caller), sig.clone(), schema.clone())
	verify {
		assert_eq!(SchemaStore::<T>::get(sig.clone()), Some(schema.clone()));
	}

	delete_schema{
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		// Dispatch a signed extrinsic.
		let name = b"Alice Data".to_vec();
		let creator = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let expiration_date: T::Moment = Default::default();
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
		let insuance_req = InsuanceRequirement{
			name: b"insuance_req".to_vec(),
			insuance_type: InsuanceType::String,
		};
		let schema: VerifiableCredentialSchema<T::Moment> = VerifiableCredentialSchema {
			name: name.clone(),
			creator: creator.clone(),
			public: false,
			creation_date: Default::default(),
			expiration_date: Some(expiration_date),
			mandatory_fields: vec![mandatory_fields.clone()],
			issuer_claims: vec![claim.clone()],
			subject_claims: vec![claim.clone()],
			credential_claims: vec![claim.clone()],
			insuance_req: insuance_req.clone(),
			metadata: b"metadata".to_vec(),
			nonce: 2,
		};
		let sig: T::Signature = sr25519::Signature::from_slice(&hex!("a6350211fcdf1d7f0c79bf0a9c296de17449ca88a899f0cd19a70b07513fc107b7d34249dba71d4761ceeec2ed6bc1305defeb96418e6869e6b6199ed0de558e")).unwrap().into();
		assert_ok!(SchemaRegistry::<T>::create_schema(RawOrigin::Signed(caller.clone()).into(), name, creator, false,
									vec![mandatory_fields], Some(expiration_date), vec![claim.clone()], 
									vec![claim.clone()], vec![claim.clone()], insuance_req.clone(), b"metadata".to_vec(), sig.clone()));
	}:  _(RawOrigin::Signed(caller.clone()), sig.clone())
	verify {
		assert_eq!(SchemaStore::<T>::get(sig.clone()), None);
	}

	create_credential{
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		let name = b"Alice Data".to_vec();
		let creator = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let context = b"Credential context".to_vec();
		let expiration_date: T::Moment = Default::default();
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
		let name = b"Alice Data".to_vec();
		let creator = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let subject = b"Credential subject".to_vec();
		let credential_holder = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let expiration_date: T::Moment = Default::default();
		let mandatory_fields = Attribute{
			name: b"name".to_vec(),
			attribute_type: AttributeType::Hex,
		};
		let schema = "VerifiableCredentialSchema".encode();
		let credential: VerifiableCredential<T::AccountId, T::Moment>  = VerifiableCredential{
			context: context.clone(),
			schema: schema.clone(),
			issuer: Some(caller.clone()),
			claim: vec![claim.clone()],
			issuance_date: Default::default(),
			expiration_date: Some(expiration_date),
			subject: subject.clone(),
			credential_holder: credential_holder.clone(),
		};
		// sign the schema.
		// must be same type as T::Signature
		let sig: T::Signature = sr25519::Signature::from_slice(&hex!("a6350211fcdf1d7f0c79bf0a9c296de17449ca88a899f0cd19a70b07513fc107b7d34249dba71d4761ceeec2ed6bc1305defeb96418e6869e6b6199ed0de558e")).unwrap().into();
		// Encode and sign the schema message.
	}:  _(RawOrigin::Signed(caller.clone()), context.clone(), schema.clone(), 
				Some(caller.clone()), vec![claim.clone()],  Some(expiration_date), 
				subject.clone(), credential_holder.clone(),sig.clone() )
	verify {
		assert_eq!(CredentialStore::<T>::get(sig.clone()), Some(credential.clone()));
	}
	update_credential{
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		let name = b"Alice Data".to_vec();
		let creator = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let context = b"Credential context".to_vec();
		let expiration_date: T::Moment = Default::default();
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
		let name = b"Alice Data".to_vec();
		//let creator = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		//let creator2 = b"did:serv:7HDx7jPsiED6n48eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let subject = b"Credential subject".to_vec();
		let credential_holder = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let credential_holder2 = b"did:serv:7JDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let expiration_date: T::Moment = Default::default();
		let mandatory_fields = Attribute{
			name: b"name".to_vec(),
			attribute_type: AttributeType::Hex,
		};
		// Encode and sign the credential.
		let schema = "VerifiableCredentialSchema".encode();
		let credential: VerifiableCredential<T::AccountId, T::Moment>  = VerifiableCredential{
			context: context.clone(),
			schema: schema.clone(),
			issuer: Some(caller.clone()),
			claim: vec![claim.clone()],
			issuance_date: Default::default(),
			expiration_date: Some(expiration_date),
			subject: subject.clone(),
			credential_holder: credential_holder.clone(),
		};
		// sign the schema.
		// must be same type as T::Signature
		let sig: T::Signature = sr25519::Signature::from_slice(&hex!("a6350211fcdf1d7f0c79bf0a9c296de17449ca88a899f0cd19a70b07513fc107b7d34249dba71d4761ceeec2ed6bc1305defeb96418e6869e6b6199ed0de558e")).unwrap().into();
		assert_ok!(SchemaRegistry::<T>::create_credential(RawOrigin::Signed(caller.clone()).into(), context.clone(), schema.clone(), 
				Some(caller.clone()), vec![claim.clone()],  Some(expiration_date), 
				subject.clone(), credential_holder2.clone(),sig.clone()));
		// Encode and sign the schema message.
	}:  _(RawOrigin::Signed(caller.clone()), sig.clone(), credential.clone())
	verify {
		assert_eq!(CredentialStore::<T>::get(sig.clone()), Some(credential.clone()));
	}
	delete_credential{
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		let name = b"Alice Data".to_vec();
		let creator = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let context = b"Credential context".to_vec();
		let expiration_date: T::Moment = Default::default();
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
		let name = b"Alice Data".to_vec();
		//let creator = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		//let creator2 = b"did:serv:7HDx7jPsiED6n48eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let subject = b"Credential subject".to_vec();
		let credential_holder = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let credential_holder2 = b"did:serv:7JDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let expiration_date: T::Moment = Default::default();
		let mandatory_fields = Attribute{
			name: b"name".to_vec(),
			attribute_type: AttributeType::Hex,
		};
		// Encode and sign the credential.
		let schema = "VerifiableCredentialSchema".encode();
		let credential: VerifiableCredential<T::AccountId, T::Moment>  = VerifiableCredential{
			context: context.clone(),
			schema: schema.clone(),
			issuer: Some(caller.clone()),
			claim: vec![claim.clone()],
			issuance_date: Default::default(),
			expiration_date: Some(expiration_date),
			subject: subject.clone(),
			credential_holder: credential_holder.clone(),
		};
		// sign the schema.
		// must be same type as T::Signature
		let sig: T::Signature = sr25519::Signature::from_slice(&hex!("a6350211fcdf1d7f0c79bf0a9c296de17449ca88a899f0cd19a70b07513fc107b7d34249dba71d4761ceeec2ed6bc1305defeb96418e6869e6b6199ed0de558e")).unwrap().into();
		assert_ok!(SchemaRegistry::<T>::create_credential(RawOrigin::Signed(caller.clone()).into(), context.clone(), schema.clone(), 
				Some(caller.clone()), vec![claim.clone()],  Some(expiration_date), 
				subject.clone(), credential_holder2.clone(),sig.clone()));
		// Encode and sign the schema message.
	}:  _(RawOrigin::Signed(caller.clone()), sig.clone())
	verify {
		assert_eq!(CredentialStore::<T>::get(sig.clone()), None);
	}

	impl_benchmark_test_suite!(SchemaRegistry, crate::mock::new_test_ext(), crate::mock::Test);
}
