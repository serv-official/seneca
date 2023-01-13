//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	create_schema{
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		let name = b"Alice Data".to_vec();
		let creator = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let expiration_date = Timestamp::now();
		let mandatory_fields = Attribute{
			name: b"name".to_vec(),
			attribute_type: AttributeType::Hex,
		};
		let issuer_claims = Claim{
			property: b"property".to_vec(),
			value: b"value".to_vec(),
			schema_id: None,
			claim_type: ClaimType::IssuerClaim,
		};
		let subject_claims = Claim{
			property: b"property".to_vec(),
			value: b"value".to_vec(),
			schema_id: None,
			claim_type: ClaimType::SubjectClaim,
		};
		let credential_claims = Claim{
			property: b"property".to_vec(),
			value: b"value".to_vec(),
			schema_id: None,
			claim_type: ClaimType::CredentialClaim,
		};
		let account_pair = account_pair("Alice");
		// Encode and sign the schema message.
		let data_sig = account_pair.sign(&creator);
		let schema = VerifiableCredentialSchema {
			id: 0,
			name: name.clone(),
			creator: creator.clone(),
			creation_date: Timestamp::now(),
			expiration_date: Some(expiration_date),
			mandatory_fields: vec![mandatory_fields.clone()],
			issuer_claims: issuer_claims.clone(),
			subject_claims: subject_claims.clone(),
			credential_claims: credential_claims.clone(),
			nonce: 2,
			signature: data_sig.clone(),
		};
	}:  _(RawOrigin::Signed(caller), 0, name.clone(), creator.clone(), 
			vec![mandatory_fields.clone()], Some(expiration_date), issuer_claims.clone(), 
			subject_claims.clone(), credential_claims.clone(), data_sig.clone())
	verify {
		assert_eq!(SchemaRegistry::<T>::get(0), Some(schema));
	}
	update_schema{
		let s in 0 .. 100;
		// Dispatch a signed extrinsic.
		let name = b"Alice Data".to_vec();
		let creator = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
		let expiration_date = Timestamp::now();
		let mandatory_fields = Attribute{
			name: b"name".to_vec(),
			attribute_type: AttributeType::Hex,
		};
		let issuer_claims = Claim{
			property: b"property".to_vec(),
			value: b"value".to_vec(),
			schema_id: None,
			claim_type: ClaimType::IssuerClaim,
		};
		let subject_claims = Claim{
			property: b"property".to_vec(),
			value: b"value".to_vec(),
			schema_id: None,
			claim_type: ClaimType::SubjectClaim,
		};
		let credential_claims = Claim{
			property: b"property".to_vec(),
			value: b"value".to_vec(),
			schema_id: None,
			claim_type: ClaimType::CredentialClaim,
		};
		let account_pair = account_pair("Alice");
		let account_pub = account_key("Alice");
		// Encode and sign the schema message.
		let data_sig = account_pair.sign(&creator);
		let schema = VerifiableCredentialSchema {
			id: 0,
			name: name.clone(),
			creator: creator.clone(),
			creation_date: Timestamp::now(),
			expiration_date: Some(expiration_date),
			mandatory_fields: vec![mandatory_fields.clone()],
			issuer_claims: issuer_claims.clone(),
			subject_claims: subject_claims.clone(),
			credential_claims: credential_claims.clone(),
			nonce: 2,
			signature: data_sig.clone(),
		};

	}:  _(RawOrigin::Signed(caller), 0, schema)
	verify {
		assert_eq!(SchemaRegistry::<T>::get(0), Some(schema));
	}

	delete_schema{
		let s in 0 .. 100;
	// Dispatch a signed extrinsic.
	let name = b"Alice Data".to_vec();
	let creator = b"did:serv:5HDx7jPsiED6n47eNfERrBBRHZb59jVW6UMZZMTSBpikzvhX".to_vec();
	let expiration_date = Timestamp::now();
	let mandatory_fields = Attribute{
		name: b"name".to_vec(),
		attribute_type: AttributeType::Hex,
	};
	let issuer_claims = Claim{
		property: b"property".to_vec(),
		value: b"value".to_vec(),
		schema_id: None,
		claim_type: ClaimType::IssuerClaim,
	};
	let subject_claims = Claim{
		property: b"property".to_vec(),
		value: b"value".to_vec(),
		schema_id: None,
		claim_type: ClaimType::SubjectClaim,
	};
	let credential_claims = Claim{
		property: b"property".to_vec(),
		value: b"value".to_vec(),
		schema_id: None,
		claim_type: ClaimType::CredentialClaim,
	};
	let account_pair = account_pair("Alice");
	let account_pub = account_key("Alice");
	// Encode and sign the schema message.
	let data_sig = account_pair.sign(&creator);
	assert_ok!(SchemaRegistry::create_schema(RawOrigin::Signed(account_pub).into(), 0, name, creator, 
								vec![mandatory_fields], Some(expiration_date), issuer_claims, 
								subject_claims, credential_claims, data_sig));
	}:  _(RawOrigin::Signed(caller), 0)
	verify {
		assert_eq!(SchemaRegistry::<T>::get(0), None);
	}
	impl_benchmark_test_suite!(SchemaRegistry, crate::mock::new_test_ext(), crate::mock::Test);
}
