use reqwest::{header, Client, Method, RequestBuilder, Url};
use serde_json::Value;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Method;

    #[tokio::test]
    async fn test_send_request() {
        let url = "https://jsonplaceholder.typicode.com/posts/1";
        let method = Method::GET;
        let api_key: Option<&str> = None;
        let custom_headers: Option<HashMap<String, String>> = None;

        let result = send_request(url, method, api_key, custom_headers).await;

        match result {
            Ok(json_response) => {
                assert!(json_response.is_object());
                assert_eq!(json_response["id"], 1);
                assert_eq!(json_response["userId"], 1);
                assert!(json_response["title"].is_string());
                assert!(json_response["body"].is_string());
            }
            Err(e) => panic!("Test failed with error: {}", e),
        }
    }
}
