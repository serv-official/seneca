use crate::mock::*;
use crate::types::*;
use codec::Encode;
use frame_support::assert_ok;
use frame_system::RawOrigin;
use http::StatusCode;
use httpmock::{Method, MockServer};
use sp_core::Pair;
use sp_runtime::traits::IdentifyAccount;

#[test]
fn it_works_for_get_requests() {
	new_test_ext().execute_with(|| {
		// Start a mock server.
		let server = MockServer::start();

		// Define the mock endpoint.
		let mock = server.mock(|when, then| {
			when.method(Method::GET)
				.path("/test")
				.header("AUTHORIZATION", "Bearer test_api_key");
			then.status(StatusCode::OK)
				.json_body_obj(&serde_json::json!({ "status": "success" }));
		});

		// Test the send_request function.
		let url = server.url("/test");
		let method = http::Method::GET;
		let api_key = Some("test_api_key");
		let custom_headers = None;
		let body = None;

		let result = OffchainWorker::send_request(&url, method, api_key, custom_headers, body);

		// Assert that the request was successful.
		assert_ok!(OffchainWorker::send_request(&url, method, api_key, custom_headers, body));
		// Assert that the mock server received the request as expected.
		mock.assert();
		// Assert that the response JSON object is correct.
		let json_response = result.unwrap();
		assert_eq!(
			OffchainWorker::send_request(&url, method, api_key, custom_headers, body).unwrap()
				["status"],
			"success"
		);
	})
}
