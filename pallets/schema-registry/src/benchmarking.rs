//! Benchmarking setup for pallet-template

use super::*;
#[allow(unused)]
use crate::Pallet as SchemaRegistry;
use sp_core::Encode;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use scale_info::prelude::vec;
use frame_support::assert_ok;
use sp_core::Pair;
use crate::types::*;

pub fn account_pair(s: &str) -> sp_core::sr25519::Pair {
    sp_core::sr25519::Pair::from_string(&format!("//{}", s), None).expect("static values are valid; qed")
}

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

		// Encode and sign the schema message.
		let account_pub = account_pair("Alice");
		let schema: VerifiableCredentialSchema< T::Moment> = VerifiableCredentialSchema {
			name: name.clone(),
			creator: creator.clone(),
			public: false,
			creation_date: Default::default(),
			expiration_date: Some(expiration_date),
			mandatory_fields: vec![mandatory_fields.clone()],
			issuer_claims: claim.clone(),
			subject_claims: claim.clone(),
			credential_claims: claim.clone(),
			nonce: 2,
		};
		// sign the schema.
		// must be same type as T::Signature
		let sig: T::Signature = account_pub.sign(&schema.encode()).into();
		// Encode and sign the schema message.
	}:  _(RawOrigin::Signed(caller), name.clone(), creator.clone(), false,
			vec![mandatory_fields.clone()], Some(expiration_date), claim.clone(), 
			claim.clone(), claim.clone(), sig.clone() )
	verify {
		assert_eq!(SchemaStore::<T>::get(sig.clone()), Some(schema));
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
		let schema: VerifiableCredentialSchema<T::Moment> = VerifiableCredentialSchema {
			name: name.clone(),
			creator: creator.clone(),
			public: false,
			creation_date: Default::default(),
			expiration_date: Some(expiration_date),
			mandatory_fields: vec![mandatory_fields.clone()],
			issuer_claims: claim.clone(),
			subject_claims: claim.clone(),
			credential_claims: claim.clone(),
			nonce: 2,
		};
		let public = account_pair("Alice");
		let sig: T::Signature = public.sign(&schema.encode()).into();

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
		let schema: VerifiableCredentialSchema<T::Moment> = VerifiableCredentialSchema {
			name: name.clone(),
			creator: creator.clone(),
			public: false,
			creation_date: Default::default(),
			expiration_date: Some(expiration_date),
			mandatory_fields: vec![mandatory_fields.clone()],
			issuer_claims: claim.clone(),
			subject_claims: claim.clone(),
			credential_claims: claim.clone(),
			nonce: 2,
		};
		let public = account_pair("Alice");
		let sig: T::Signature = public.sign(&schema.encode()).into();
		assert_ok!(SchemaRegistry::<T>::create_schema(RawOrigin::Signed(caller.clone()).into(), name, creator, false,
									vec![mandatory_fields], Some(expiration_date), claim.clone(), 
									claim.clone(), claim.clone(), sig.clone()));
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
		// Encode and sign the credential.
		let account_pub = account_pair("Alice");
		let schema = "VerifiableCredentialSchema".encode();
		let credential: VerifiableCredential<T::AccountId, T::Moment>  = VerifiableCredential{
			context: context.clone(),
			schema: schema.clone(),
			issuer: Some(caller.clone()),
			claim: claim.clone(),
			issuance_date: Default::default(),
			expiration_date: Some(expiration_date),
			subject: subject.clone(),
			credential_holder: credential_holder.clone(),
		};
		// sign the schema.
		// must be same type as T::Signature
		let sig: T::Signature = account_pub.sign(&credential.encode()).into();
		// Encode and sign the schema message.
	}:  _(RawOrigin::Signed(caller.clone()), context.clone(), schema.clone(), 
				Some(caller.clone()), claim.clone(),  Some(expiration_date), 
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
		let account_pub = account_pair("Alice");
		let schema = "VerifiableCredentialSchema".encode();
		let credential: VerifiableCredential<T::AccountId, T::Moment>  = VerifiableCredential{
			context: context.clone(),
			schema: schema.clone(),
			issuer: Some(caller.clone()),
			claim: claim.clone(),
			issuance_date: Default::default(),
			expiration_date: Some(expiration_date),
			subject: subject.clone(),
			credential_holder: credential_holder.clone(),
		};
		// sign the schema.
		// must be same type as T::Signature
		let sig: T::Signature = account_pub.sign(&credential.encode()).into();
		assert_ok!(SchemaRegistry::<T>::create_credential(RawOrigin::Signed(caller.clone()).into(), context.clone(), schema.clone(), 
				Some(caller.clone()), claim.clone(),  Some(expiration_date), 
				subject.clone(), credential_holder2.clone(),sig.clone()));
		// Encode and sign the schema message.
	}:  _(RawOrigin::Signed(caller.clone()), sig.clone(), credential.clone())
	verify {
		assert_eq!(CredentialStore::<T>::get(sig.clone()), Some(credential.clone()));
	}

	impl_benchmark_test_suite!(SchemaRegistry, crate::mock::new_test_ext(), crate::mock::Test);
}
