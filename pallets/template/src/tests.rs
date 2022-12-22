use crate::{mock::*, types::DIDData};
use frame_support::assert_ok;


///create did with DID data
#[test]
fn it_works_for_create() {
	new_test_ext().execute_with(|| {

		let did = serde_json::json!({
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
		let data = DIDData{
			id: 00001,
			data: did.clone(),
		};
		
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create(RuntimeOrigin::signed(1), did.clone(), data.clone()));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::store_did(did.clone()), Some(data.clone()));
	});
}

#[test]
fn it_works_for_update() {
	new_test_ext().execute_with(|| {

		let did = serde_json::json!({
			 "credential":{
				"credentialSubject": {
					"name": "The greatest",
					"id": "123"
				},
				"issuer": {
					"id": "did:web:veramo-update.herokuapp.com"
				},
				"id": "12345",
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
		let data = DIDData{
			id: 00001,
			data: did.clone(),
		};
		
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::update(RuntimeOrigin::signed(1), did.clone(), data.clone()));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::store_did(did.clone()), Some(data.clone()));
	});
}


