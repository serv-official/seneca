use core::str::FromStr;
use sp_core::H256;
use crate::mock::*;
use frame_support::assert_ok;
use sp_core::sr25519;
use crate::types::{Registry, RegistryType};
use frame_system::RawOrigin;

#[test]
fn it_works_for_create_schema() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
			let hash = H256::from_str("0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d").unwrap();
			let address = sr25519::Public::from_h256(hash);
			let data = serde_json::json!({
				"credential":{
				"credentialSubject": {
					"name": "The greatest",
					"id": "123"
				},
				"issuer": {
					"id": "did:web:veramo-update.herokuapp.com"
				},
				"id": "1234",
				"type": [
					"VerifiableCredential",
					"test"
				],
				"@context": [
					"https://www.w3.org/2018/credentials/v1",
					"string"
				],
				"issuanceDate": "2022-10-25T16:07:03.000Z",
				"proof": {
					"type": "JwtProof2020",
					"jwt": "eyJhbGciOiJFUzI1NksiLCJ0eXAiOiJKV1QifQ.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSIsInN0cmluZyJdLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIiwidGVzdCJdLCJjcmVkZW50aWFsU3ViamVjdCI6eyJuYW1lIjoiVGhlIGdyZWF0ZXN0In19LCJzdWIiOiIxMjMiLCJqdGkiOiIxMjM0IiwibmJmIjoxNjY2NzE0MDIzLCJpc3MiOiJkaWQ6d2ViOnZlcmFtby11cGRhdGUuaGVyb2t1YXBwLmNvbSJ9.h5boNBuP-wQX8pN72WaPsRD4k5zRYLgsAur0y-Gbfd3cDHskPLpd4TTuvlGxT3FVrdrU4ux59WAdca2Hv4BCyw"
				}
			}
		}).to_string().into_bytes();

		let registry = Registry {
			id: 1,
			address,
			registry_type: RegistryType::CredentialRegsitry,
			data,
		};
		// Dispatch a signed create schema extrinsic.
		assert_ok!(SchemaRegistry::create(RawOrigin::Root.into(), hash, registry.clone()));
		// Read pallet storage and assert an expected result.
		assert_eq!(SchemaRegistry::store_regisrty(hash), Some(registry.clone()));
	});
}

#[test]
fn it_works_for_update_schema() {
	new_test_ext().execute_with(|| {
			let hash = H256::from_str("0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d").unwrap();
			let address = sr25519::Public::from_h256(hash);
			let old_data = serde_json::json!({
				"credential":{
				"credentialSubject": {
					"name": "The greatest",
					"id": "123"
				},
				"issuer": {
					"id": "did:web:veramo-update.herokuapp.com"
				},
				"id": "1234",
				"type": [
					"VerifiableCredential",
					"test"
				],
				"@context": [
					"https://www.w3.org/2018/credentials/v1",
					"string"
				],
				"issuanceDate": "2022-10-25T16:07:03.000Z",
				"proof": {
					"type": "JwtProof2020",
					"jwt": "eyJhbGciOiJFUzI1NksiLCJ0eXAiOiJKV1QifQ.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSIsInN0cmluZyJdLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIiwidGVzdCJdLCJjcmVkZW50aWFsU3ViamVjdCI6eyJuYW1lIjoiVGhlIGdyZWF0ZXN0In19LCJzdWIiOiIxMjMiLCJqdGkiOiIxMjM0IiwibmJmIjoxNjY2NzE0MDIzLCJpc3MiOiJkaWQ6d2ViOnZlcmFtby11cGRhdGUuaGVyb2t1YXBwLmNvbSJ9.h5boNBuP-wQX8pN72WaPsRD4k5zRYLgsAur0y-Gbfd3cDHskPLpd4TTuvlGxT3FVrdrU4ux59WAdca2Hv4BCyw"
				}
			}
		}).to_string().into_bytes();
		let data = serde_json::json!({
				"credential":{
				"credentialSubject": {
					"name": "The greatest",
					"id": "1234"
				},
				"issuer": {
					"id": "did:web:veramo-update.herokuapp.com"
				},
				"id": "1234",
				"type": [
					"VerifiableCredential",
					"test"
				],
				"@context": [
					"https://www.w3.org/2018/credentials/v1",
					"string"
				],
				"issuanceDate": "2022-11-25T16:07:03.000Z",
				"proof": {
					"type": "JwtProof2022",
					"jwt": "eyJhbGciOiJFUzI1NksiLCJ0eXAiOiJKV1QifQ.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSIsInN0cmluZyJdLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIiwidGVzdCJdLCJjcmVkZW50aWFsU3ViamVjdCI6eyJuYW1lIjoiVGhlIGdyZWF0ZXN0In19LCJzdWIiOiIxMjMiLCJqdGkiOiIxMjM0IiwibmJmIjoxNjY2NzE0MDIzLCJpc3MiOiJkaWQ6d2ViOnZlcmFtby11cGRhdGUuaGVyb2t1YXBwLmNvbSJ9.h5boNBuP-wQX8pN72WaPsRD4k5zRYLgsAur0y-Gbfd3cDHskPLpd4TTuvlGxT3FVrdrU4ux59WAdca2Hv4BCyw"
				}
			}
		}).to_string().into_bytes();
		let old_registry = Registry {
			id: 1,
			address,
			registry_type: RegistryType::CredentialRegsitry,
			data: old_data,
		};
		let registry = Registry {
			id: 1,
			address,
			registry_type: RegistryType::CredentialRegsitry,
			data,
		};
		// Dispatch a signed extrinsic.
		assert_ok!(SchemaRegistry::create(RawOrigin::Root.into(), hash, old_registry.clone()));
		assert_ok!(SchemaRegistry::update(RawOrigin::Root.into(), hash, old_registry, registry.clone()));
		// Read pallet storage and assert an expected result.
		assert_eq!(SchemaRegistry::store_regisrty(hash), Some(registry.clone()));
	});
}

#[test]
fn it_works_for_delete_schema() {
	new_test_ext().execute_with(|| {
			let hash = H256::from_str("0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d").unwrap();
			let address = sr25519::Public::from_h256(hash);
			let data = serde_json::json!({
				"credential":{
				"credentialSubject": {
					"name": "The greatest",
					"id": "123"
				},
				"issuer": {
					"id": "did:web:veramo-update.herokuapp.com"
				},
				"id": "1234",
				"type": [
					"VerifiableCredential",
					"test"
				],
				"@context": [
					"https://www.w3.org/2018/credentials/v1",
					"string"
				],
				"issuanceDate": "2022-10-25T16:07:03.000Z",
				"proof": {
					"type": "JwtProof2020",
					"jwt": "eyJhbGciOiJFUzI1NksiLCJ0eXAiOiJKV1QifQ.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSIsInN0cmluZyJdLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIiwidGVzdCJdLCJjcmVkZW50aWFsU3ViamVjdCI6eyJuYW1lIjoiVGhlIGdyZWF0ZXN0In19LCJzdWIiOiIxMjMiLCJqdGkiOiIxMjM0IiwibmJmIjoxNjY2NzE0MDIzLCJpc3MiOiJkaWQ6d2ViOnZlcmFtby11cGRhdGUuaGVyb2t1YXBwLmNvbSJ9.h5boNBuP-wQX8pN72WaPsRD4k5zRYLgsAur0y-Gbfd3cDHskPLpd4TTuvlGxT3FVrdrU4ux59WAdca2Hv4BCyw"
				}
			}
		}).to_string().into_bytes();

		let registry = Registry {
			id: 1,
			address,
			registry_type: RegistryType::CredentialRegsitry,
			data,
		};
		// Dispatch a signed create schema extrinsic.
		assert_ok!(SchemaRegistry::create(RawOrigin::Root.into(), hash, registry.clone()));
		// Dispatch a signed extrinsic.
		assert_ok!(SchemaRegistry::delete(RawOrigin::Root.into(), hash.clone()));
		// Read pallet storage and assert an expected result.
		assert_eq!(SchemaRegistry::store_regisrty(hash), None);
	});
}
