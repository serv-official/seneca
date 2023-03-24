#[cfg(test)]
mod tests {
    use super::*;
    use http::{header, Method};
    use sp_core::offchain::{
        testing::{OffchainState, PoolState},
        OffchainExt, TransactionPoolExt,
    };
    use substrate_test_utils::TestExternalities;

    #[test]
    fn test_send_request() {
        let mut ext = TestExternalities::default();
        let (offchain, _) = OffchainState::new();
        let (pool, _) = PoolState::new();

        ext.register_extension(OffchainExt::new(offchain));
        ext.register_extension(TransactionPoolExt::new(pool));

        ext.execute_with(|| {
                      let url = "https://jsonplaceholder.typicode.com/posts/1";
            let method = Method::GET;
            let api_key: Option<&str> = None;
            let custom_headers: Option<Vec<(HeaderName, HeaderValue)>> = None;

            let result = send_request(url, method, api_key, custom_headers);

            match result {
                Ok(json_response) => {
                    assert!(json_response.is_object());
                    assert_eq!(json_response["id"], 1);
                    assert_eq!(json_response["userId"], 1);
                    assert!(json_response["title"].is_string());
                    assert!(json_response["body"].is_string());
                }
                Err(e) => panic!("Test failed with error: {:?}", e),
            }
        });
    }
}




