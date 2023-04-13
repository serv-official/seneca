use crate::mock::*;

use frame_support::traits::OnTimestampSet;
use frame_support::traits::{ConstU16, ConstU64};
use frame_support::{assert_err, assert_ok};
use frame_system::GenesisConfig;
use frame_system::RawOrigin;
use sp_core::Pair;
use sp_runtime::traits::IdentifyAccount;
#[test]
fn it_works_for_make_get_request() {
	new_test_ext().execute_with(|| {
	

		// let account_pair = account_pair("Alice");
		// let signer = account_pair.public();
		// let account_id = format!("did:seneca:{}", account_pair.public().into_account());
		// // Dispatch a signed extrinsic
		// // Dispatch a signed extrinsic
		// let url = b"https://example.com".to_vec();


		// assert_ok!(OffchainWorker::make_get_request(
		// 	RawOrigin::Signed(account_id.clone()).into(),
		// 	url.clone(),
		// ));

		// // Check that an event was emitted with the correct response
		// let event = pallet_offchain_worker::Event::HttpGetResponse(
		// 	account_id,
		// 	// Replace this Vec<u8> with the expected response body as bytes
		// 	b"response body".to_vec(),
		// );
		// assert!(System::events().iter().any(|record| record.event == event));

		// // Invalid URL
		// let invalid_url = b"invalid-url".to_vec();
		// assert_err!(
		// 	OffchainWorker::make_get_request(RawOrigin::Signed(account_id).into(), invalid_url),
		// 	crate::Error::<Test>::VecToUrlConversionFailed
		// );
	});
}
#[test]
fn it_works_for_make_post_request() {
	new_test_ext().execute_with(|| {
		// // assert_eq!(1, 1);
		// // Create Alice's account
		// let account_pair = account_pair("Alice");
		// let signer = account_pair.public();
		// let account_id = format!("did:seneca:{}", account_pair.public().into_account());
		// // // Dispatch a signed extrinsic
		// // // Dispatch a signed extrinsic
		// let url = b"https://example.com".to_vec();

		
		// assert_ok!(OffchainWorker::make_get_request(
		// 	RawOrigin::Signed(account_id.clone()).into(),
		// 	url.clone(),
		// ));

		// // Check that an event was emitted with the correct response
		// let event = pallet_offchain_worker::Event::HttpGetResponse(
		// 	account_id,
		// 	// Replace this Vec<u8> with the expected response body as bytes
		// 	b"response body".to_vec(),
		// );
		// assert!(System::events().iter().any(|record| record.event == event));

		// // Invalid URL
		// let invalid_url = b"invalid-url".to_vec();
		// assert_err!(
		// 	OffchainWorker::make_get_request(RawOrigin::Signed(account_id).into(), invalid_url),
		// 	crate::Error::<Test>::VecToUrlConversionFailed
		// );
	});
}
