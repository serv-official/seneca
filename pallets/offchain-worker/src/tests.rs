#[cfg(test)]
mod tests {
    use super::*;
    use sc_service::Configuration;
    use sp_core::offchain::{OffchainWorkerExt, testing};
    use sp_runtime::testing::TestRuntime;
    use substrate_test_utils::new_test_ext;

    #[test]
    fn test_send_request() {
        let (mut test_ext, _offchain_state) = new_test_ext(
            Configuration {
                offchain_worker: Default::default(),
                ..Default::default()
            },
            TestRuntime::default(),
        );

        test_ext.execute_with(|| {
            // Set up mock HTTP server
            let mock_request = |url: &str, _request: &testing::TestOffchainExt::Call| -> testing::Result {
                if url.ends_with("/test") {
                    Ok(testing::HttpResponse {
                        status_code: 200,
                        body: "{\"success\": true}".as_bytes().to_vec(),
                        headers: None,
                    })
                } else {
                    Ok(testing::HttpResponse {
                        status_code: 404,
                        body: "{\"error\": \"Not found\"}".as_bytes().to_vec(),
                        headers: None,
                    })
                }
            };
    
                sp_io::offchain::set_http_request_handler(mock_request);
    
                // Call send_request function with test parameters
                let url = "https://localhost/test";
                let method = http::Method::Get;
                let api_key: Option<&str> = None;
                let custom_headers: Option<Vec<(String, String)>> = None;
                let body: Option<Vec<u8>> = None;
    
                let result = send_request(&url, method, api_key, custom_headers, body).unwrap();
                let expected_result = JsonValue::Object(
                    vec![("success".to_string(), JsonValue::Bool(true))]
                        .into_iter()
                        .collect(),
                );
    
                // Assert the result is as expected
                assert_eq!(result, expected_result);
            });
        }
    }

    