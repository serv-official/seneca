//! Benchmarking setup for pallet-template
use super::*;
#[allow(unused)]
use crate::Pallet as SchemaRegistry;
use codec::Encode;
use sp_core::{Pair, sr25519};
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
		let asset_id: T::SchemaId = Default::default();
		let keypair = sr25519::Pair::from_string("//Alice", None).unwrap();
		let pub_key: T::Public = keypair.public().into();

		let schema: VerifiableCredentialSchema<T::Public, T::Moment> = VerifiableCredentialSchema {
			name: name.clone(),
			creator: pub_key.clone(),
			public: false,
			creation_date: Default::default(),
			expiration_date: Some(expiration_date),
			mandatory_fields: vec![mandatory_fields.clone()],
			issuer_claims: vec![claim.clone()],
			subject_claims: vec![claim.clone()],
			credential_claims: vec![claim.clone()],
			metadata: b"metadata".to_vec(),
			nonce,
		};

		// sign the schema in benchmarks
		let sig: T::Signature = keypair.sign(&schema.encode()).into();

		// Encode and sign the schema message.
	}:  _(RawOrigin::Signed(caller), name.clone(), pub_key, false,
			vec![mandatory_fields.clone()], Some(expiration_date), vec![claim.clone()], 
			vec![claim.clone()], vec![claim.clone()], b"metadata".to_vec(), sig.clone(), asset_id.clone(),nonce )
	verify {
		//assert that the schema stored is different from the one created since the nonce is different.
		assert_eq!(SchemaStore::<T>::get(asset_id.clone()), Some((sig, schema)));
	}
	update_schema{
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
		let nonce = 2u64;
		let claim = Claim{
			property: b"property".to_vec(),
			value: b"value".to_vec(),
			schemaid: None,
			claim_type: ClaimType::CredentialClaim,
			issuance_requirement: Some(vec![issuance_req.clone()]),
		};
		//create random schema_id
		let asset_id: T::SchemaId = Default::default();
		let keypair = sr25519::Pair::from_string("//Alice", None).unwrap();
		let pub_key: T::Public = keypair.public().into();

		let schema: VerifiableCredentialSchema<T::Public, T::Moment> = VerifiableCredentialSchema {
			name: name.clone(),
			creator: pub_key.clone(),
			public: false,
			creation_date: Default::default(),
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
			creator: pub_key.clone(),
			public: false,
			creation_date: Default::default(),
			expiration_date: Some(expiration_date),
			mandatory_fields: vec![mandatory_fields.clone()],
			issuer_claims: vec![claim.clone()],
			subject_claims: vec![claim.clone()],
			credential_claims: vec![claim.clone()],
			metadata: b"metadata2".to_vec(),
			nonce,
		};

		// sign the schema in benchmarks
		let sig: T::Signature = keypair.sign(&schema.encode()).into();

		assert_ok!(SchemaRegistry::<T>::create_schema(RawOrigin::Signed(caller.clone()).into(), name, pub_key, false,
									vec![mandatory_fields], Some(expiration_date), vec![claim.clone()], 
									vec![claim.clone()], vec![claim.clone()], b"metadata".to_vec(), sig.clone(), asset_id.clone(), nonce));
	}:  _(RawOrigin::Signed(caller), asset_id.clone(), (sig.clone(), updated_schema.clone()))
	verify {
		assert_eq!(SchemaStore::<T>::get(asset_id.clone()), Some((sig.clone(), updated_schema.clone())));
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
		let asset_id: T::SchemaId = Default::default();

		// sign the schema in benchmarks
		let keypair = sr25519::Pair::from_string("//Alice", None).unwrap();
		let pub_key: T::Public = keypair.public().into();

		let schema: VerifiableCredentialSchema<T::Public, T::Moment> = VerifiableCredentialSchema {
			name: name.clone(),
			creator: pub_key.clone(),
			public: false,
			creation_date: Default::default(),
			expiration_date: Some(expiration_date),
			mandatory_fields: vec![mandatory_fields.clone()],
			issuer_claims: vec![claim.clone()],
			subject_claims: vec![claim.clone()],
			credential_claims: vec![claim.clone()],
			metadata: b"metadata".to_vec(),
			nonce,
		};

		let sig: T::Signature = keypair.sign(&schema.encode()).into();

		assert_ok!(SchemaRegistry::<T>::create_schema(RawOrigin::Signed(caller.clone()).into(), name, pub_key, false,
									vec![mandatory_fields], Some(expiration_date), vec![claim.clone()], 
									vec![claim.clone()], vec![claim.clone()], b"metadata".to_vec(), sig.clone(), asset_id.clone(), nonce));
	}:  _(RawOrigin::Signed(caller.clone()), asset_id.clone())
	verify {
		assert_eq!(SchemaStore::<T>::get(asset_id.clone()), None);
	}

	create_credential{
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		let name = b"Alice Data".to_vec();
		let creator = b"did:seneca:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let context = b"Credential context".to_vec();
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
		let name = b"Alice Data".to_vec();
		let creator = b"did:seneca:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let subject = b"Credential subject".to_vec();
		let credential_holder = b"did:seneca:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let expiration_date: T::Moment = Default::default();
		let mandatory_fields = Attribute{
			name: b"name".to_vec(),
			attribute_type: AttributeType::Hex,
		};
		let issuer = b"did:seneca:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let subject = Subject{
			id: b"did:seneca:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec(),
			claim: vec![claim.clone()],
		};
		let nonce = 2u64;
		let schema = "VerifiableCredentialSchema".encode();
		//create random credential_id
		let asset_id: T::CredentialId = Default::default();
		// sign the credential in benchmarks
		let keypair = sr25519::Pair::from_string("//Alice", None).unwrap();
		let pub_key: T::Public = keypair.public().into();

		let credential: VerifiableCredential<T::Public, T::Moment>  = VerifiableCredential{
			context: context.clone(),
			schema: schema.clone(),
			issuer: pub_key.clone(),
			issuance_date: Some(Default::default()),
			expiration_date: Some(expiration_date),
			subject: subject.clone(),
			credential_holder: credential_holder.clone(),
			nonce: nonce.clone()
		};

		let sig: T::Signature = keypair.sign(&credential.encode()).into();

	}:  _(RawOrigin::Signed(caller.clone()), context.clone(), schema.clone(), 
				pub_key.clone(),  Some(expiration_date),subject.clone(), credential_holder.clone(), sig.clone(), nonce, asset_id.clone() )
	verify {
		assert_eq!(CredentialStore::<T>::get(asset_id.clone()), Some((sig.clone(), credential.clone())));
	}
	update_credential{
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		let name = b"Alice Data".to_vec();
		let creator = b"did:seneca:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let context = b"Credential context".to_vec();
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
		let name = b"Alice Data".to_vec();
		let subject = b"Credential subject".to_vec();
		let credential_holder = b"did:seneca:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let credential_holder2 = b"did:seneca:7JDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let issuer = b"did:seneca:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let expiration_date: T::Moment = Default::default();
		let mandatory_fields = Attribute{
			name: b"name".to_vec(),
			attribute_type: AttributeType::Hex,
		};
		let subject = Subject{
			id: b"did:seneca:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec(),
			claim: vec![claim.clone()],
		};
		let nonce = 2u64;
		let schema = "VerifiableCredentialSchema".encode();
		//create random credential_id
		let asset_id: T::CredentialId = Default::default();
		// sign the credential in benchmarks
		let keypair = sr25519::Pair::from_string("//Alice", None).unwrap();
		let pub_key: T::Public = keypair.public().into();

		let credential: VerifiableCredential<T::Public, T::Moment>  = VerifiableCredential{
			context: context.clone(),
			schema: schema.clone(),
			issuer: pub_key.clone(),
			issuance_date: Some(Default::default()),
			expiration_date: Some(expiration_date),
			subject: subject.clone(),
			credential_holder: credential_holder.clone(),
			nonce: nonce.clone()
		};

		let credential2: VerifiableCredential<T::Public, T::Moment>  = VerifiableCredential{
			context: context.clone(),
			schema: schema.clone(),
			issuer: pub_key.clone(),
			issuance_date: Some(Default::default()),
			expiration_date: Some(expiration_date),
			subject: subject.clone(),
			credential_holder: credential_holder2.clone(),
			nonce: nonce.clone()
		};

		let sig: T::Signature = keypair.sign(&credential.encode()).into();
		let sig2: T::Signature = keypair.sign(&credential2.encode()).into();

		assert_ok!(SchemaRegistry::<T>::create_credential(RawOrigin::Signed(caller.clone()).into(), context.clone(), schema.clone(), 
				pub_key.clone(), Some(expiration_date), subject.clone(), credential_holder.clone(),sig.clone(), nonce, asset_id.clone()));
		// Encode and sign the schema message.
	}:  _(RawOrigin::Signed(caller.clone()), asset_id.clone(),(sig2.clone(), credential2.clone()))
	verify {
		assert_eq!(CredentialStore::<T>::get(asset_id.clone()), Some((sig2.clone(), credential2.clone())));
	}
	delete_credential{
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		let name = b"Alice Data".to_vec();
		let creator = b"did:seneca:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let context = b"Credential context".to_vec();
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
		let name = b"Alice Data".to_vec();
		let subject = b"Credential subject".to_vec();
		let credential_holder = b"did:seneca:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let credential_holder2 = b"did:seneca:7JDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let issuer = b"did:seneca:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let expiration_date: T::Moment = Default::default();
		let mandatory_fields = Attribute{
			name: b"name".to_vec(),
			attribute_type: AttributeType::Hex,
		};
		let subject = Subject{
			id: b"did:seneca:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec(),
			claim: vec![claim.clone()],
		};
		let nonce = 2u64;

		//create random credential_id
		let asset_id: T::CredentialId = Default::default();
		let schema = "VerifiableCredentialSchema".encode();
		// sign the credential in benchmarks
		let keypair = sr25519::Pair::from_string("//Alice", None).unwrap();
		let pub_key: T::Public = keypair.public().into();

		let credential: VerifiableCredential<T::Public, T::Moment>  = VerifiableCredential{
			context: context.clone(),
			schema: schema.clone(),
			issuer: pub_key.clone(),
			issuance_date: Some(Default::default()),
			expiration_date: Some(expiration_date),
			subject: subject.clone(),
			credential_holder: credential_holder.clone(),
			nonce: nonce.clone()
		};

		let sig: T::Signature = keypair.sign(&credential.encode()).into();

		assert_ok!(SchemaRegistry::<T>::create_credential(RawOrigin::Signed(caller.clone()).into(), context.clone(), schema.clone(), 
				pub_key, Some(expiration_date), subject.clone(), credential_holder.clone(),sig.clone(), nonce, asset_id.clone()));
		// Encode and sign the schema message.
	}:  _(RawOrigin::Signed(caller.clone()), asset_id.clone())
	verify {
		assert_eq!(CredentialStore::<T>::get(asset_id.clone()), None);
	}

	impl_benchmark_test_suite!(SchemaRegistry, crate::mock::new_test_ext(), crate::mock::Test);
}
