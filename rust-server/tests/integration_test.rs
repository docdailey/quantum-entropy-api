#[cfg(test)]
mod tests {
    use reqwest;
    use serde_json::Value;

    const BASE_URL: &str = "http://localhost:8080";

    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored
    async fn test_health_endpoint() {
        let response = reqwest::get(format!("{}/api/v1/health", BASE_URL))
            .await
            .expect("Failed to get health");
        
        assert_eq!(response.status(), 200);
        
        let json: Value = response.json().await.expect("Failed to parse JSON");
        assert_eq!(json["status"], "healthy");
    }

    #[tokio::test]
    #[ignore]
    async fn test_random_bytes() {
        let response = reqwest::get(format!("{}/api/v1/random/bytes?count=32", BASE_URL))
            .await
            .expect("Failed to get random bytes");
        
        assert_eq!(response.status(), 200);
        
        let json: Value = response.json().await.expect("Failed to parse JSON");
        assert!(json["success"].as_bool().unwrap());
        assert_eq!(json["data"]["count"], 32);
        
        // Check hex format
        let bytes = json["data"]["bytes"].as_str().unwrap();
        assert_eq!(bytes.len(), 64); // 32 bytes = 64 hex chars
    }

    #[tokio::test]
    #[ignore]
    async fn test_random_integers() {
        let response = reqwest::get(format!("{}/api/v1/random/int?min=1&max=100&count=10", BASE_URL))
            .await
            .expect("Failed to get random integers");
        
        assert_eq!(response.status(), 200);
        
        let json: Value = response.json().await.expect("Failed to parse JSON");
        assert!(json["success"].as_bool().unwrap());
        
        let integers = json["data"]["integers"].as_array().unwrap();
        assert_eq!(integers.len(), 10);
        
        // Verify all integers are in range
        for int in integers {
            let value = int.as_i64().unwrap();
            assert!(value >= 1 && value <= 100);
        }
    }
}