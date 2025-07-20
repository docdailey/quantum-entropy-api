use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// Quantum Entropy API Client for Rust
/// 
/// Example usage:
/// ```rust
/// let client = QuantumClient::new();
/// let bytes = client.get_random_bytes(32).await?;
/// println!("Random bytes: {}", bytes.bytes);
/// ```

const API_BASE: &str = "https://quantum-server.docdailey.ai";

#[derive(Debug, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    data: T,
    error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BytesData {
    pub bytes: String,
    pub count: u32,
    pub format: String,
    pub correction: String,
}

#[derive(Debug, Deserialize)]
pub struct PasswordData {
    pub password: String,
    pub length: u32,
    pub digits: bool,
    pub lowercase: bool,
    pub uppercase: bool,
    pub symbols: bool,
}

#[derive(Debug, Deserialize)]
pub struct KeyData {
    pub key: String,
    pub key_base64: String,
    pub bits: u32,
}

#[derive(Debug, Deserialize)]
pub struct UuidData {
    pub uuid: String,
}

pub struct QuantumClient {
    client: reqwest::Client,
    base_url: String,
}

impl QuantumClient {
    /// Create a new Quantum API client
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: API_BASE.to_string(),
        }
    }

    /// Create a client with custom base URL
    pub fn with_base_url(base_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
        }
    }

    /// Get random bytes
    pub async fn get_random_bytes(&self, count: u32) -> Result<BytesData, Box<dyn Error>> {
        let url = format!("{}/api/v1/random/bytes", self.base_url);
        let response: ApiResponse<BytesData> = self.client
            .get(&url)
            .query(&[("count", count)])
            .send()
            .await?
            .json()
            .await?;

        if response.success {
            Ok(response.data)
        } else {
            Err(response.error.unwrap_or_else(|| "Unknown error".to_string()).into())
        }
    }

    /// Get random bytes with options
    pub async fn get_random_bytes_with_options(
        &self,
        count: u32,
        format: &str,
        correction: &str,
    ) -> Result<BytesData, Box<dyn Error>> {
        let url = format!("{}/api/v1/random/bytes", self.base_url);
        let response: ApiResponse<BytesData> = self.client
            .get(&url)
            .query(&[
                ("count", count.to_string()),
                ("format", format.to_string()),
                ("correction", correction.to_string()),
            ])
            .send()
            .await?
            .json()
            .await?;

        if response.success {
            Ok(response.data)
        } else {
            Err(response.error.unwrap_or_else(|| "Unknown error".to_string()).into())
        }
    }

    /// Get random integers
    pub async fn get_random_integers(
        &self,
        min: i32,
        max: i32,
        count: u32,
    ) -> Result<Vec<i32>, Box<dyn Error>> {
        let url = format!("{}/api/v1/random/integers", self.base_url);
        let response: ApiResponse<Vec<i32>> = self.client
            .get(&url)
            .query(&[
                ("min", min.to_string()),
                ("max", max.to_string()),
                ("count", count.to_string()),
            ])
            .send()
            .await?
            .json()
            .await?;

        if response.success {
            Ok(response.data)
        } else {
            Err(response.error.unwrap_or_else(|| "Unknown error".to_string()).into())
        }
    }

    /// Generate a secure password
    pub async fn generate_password(
        &self,
        length: u32,
        symbols: bool,
    ) -> Result<PasswordData, Box<dyn Error>> {
        let url = format!("{}/api/v1/crypto/password", self.base_url);
        let response: ApiResponse<PasswordData> = self.client
            .get(&url)
            .query(&[
                ("length", length.to_string()),
                ("symbols", symbols.to_string()),
            ])
            .send()
            .await?
            .json()
            .await?;

        if response.success {
            Ok(response.data)
        } else {
            Err(response.error.unwrap_or_else(|| "Unknown error".to_string()).into())
        }
    }

    /// Generate a cryptographic key
    pub async fn generate_key(&self, bits: u32) -> Result<KeyData, Box<dyn Error>> {
        let url = format!("{}/api/v1/crypto/key", self.base_url);
        let response: ApiResponse<KeyData> = self.client
            .get(&url)
            .query(&[("level", bits)])
            .send()
            .await?
            .json()
            .await?;

        if response.success {
            Ok(response.data)
        } else {
            Err(response.error.unwrap_or_else(|| "Unknown error".to_string()).into())
        }
    }

    /// Generate a UUID v4
    pub async fn generate_uuid(&self) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/api/v1/crypto/uuid", self.base_url);
        let response: ApiResponse<UuidData> = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;

        if response.success {
            Ok(response.data.uuid)
        } else {
            Err(response.error.unwrap_or_else(|| "Unknown error".to_string()).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_random_bytes() {
        let client = QuantumClient::new();
        let result = client.get_random_bytes(16).await;
        assert!(result.is_ok());
        
        let bytes = result.unwrap();
        assert_eq!(bytes.count, 16);
        assert_eq!(bytes.format, "hex");
        assert_eq!(bytes.bytes.len(), 32); // 16 bytes = 32 hex chars
    }

    #[tokio::test]
    async fn test_random_integers() {
        let client = QuantumClient::new();
        let result = client.get_random_integers(1, 100, 5).await;
        assert!(result.is_ok());
        
        let integers = result.unwrap();
        assert_eq!(integers.len(), 5);
        for num in integers {
            assert!(num >= 1 && num <= 100);
        }
    }
}

/// Example usage
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = QuantumClient::new();

    // Get random bytes
    println!("🎲 Getting random bytes...");
    let bytes = client.get_random_bytes(32).await?;
    println!("Random bytes (hex): {}", bytes.bytes);
    println!("Format: {}, Count: {}", bytes.format, bytes.count);

    // Get random integers (dice roll)
    println!("\n🎲 Rolling dice...");
    let dice = client.get_random_integers(1, 6, 2).await?;
    println!("Dice roll: {:?} (total: {})", dice, dice.iter().sum::<i32>());

    // Generate password
    println!("\n🔐 Generating password...");
    let password = client.generate_password(20, true).await?;
    println!("Password: {}", password.password);

    // Generate encryption key
    println!("\n🔑 Generating 256-bit key...");
    let key = client.generate_key(256).await?;
    println!("Key (hex): {}", key.key);
    println!("Key (base64): {}", key.key_base64);

    // Generate UUID
    println!("\n🆔 Generating UUID...");
    let uuid = client.generate_uuid().await?;
    println!("UUID: {}", uuid);

    Ok(())
}