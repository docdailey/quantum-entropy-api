//! REST API endpoints

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::device::{bias_correction, QuantisDevice};
use crate::utils::RingBuffer;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(msg: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.into()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BytesQuery {
    #[serde(default = "default_count")]
    pub count: usize,
    #[serde(default = "default_format")]
    pub format: String,
    #[serde(default = "default_correction")]
    pub correction: String,
}

fn default_count() -> usize { 32 }
fn default_format() -> String { "hex".to_string() }
fn default_correction() -> String { "none".to_string() }

#[derive(Debug, Serialize)]
pub struct BytesResponse {
    pub bytes: String,
    pub count: usize,
    pub format: String,
    pub correction: String,
}

#[derive(Debug, Deserialize)]
pub struct IntegersQuery {
    pub min: i64,
    pub max: i64,
    #[serde(default = "default_int_count")]
    pub count: usize,
}

fn default_int_count() -> usize { 1 }

#[derive(Debug, Serialize)]
pub struct IntegersResponse {
    pub integers: Vec<i64>,
    pub min: i64,
    pub max: i64,
    pub count: usize,
}

pub type AppState = Arc<AppStateInner>;

pub struct AppStateInner {
    pub device: Arc<Mutex<QuantisDevice>>,
    pub buffer: Arc<RingBuffer>,
}

/// Create API routes
pub fn routes(device: Arc<Mutex<QuantisDevice>>, buffer: Arc<RingBuffer>) -> Router {
    let state = Arc::new(AppStateInner { device, buffer });

    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/random/bytes", get(random_bytes))
        .route("/random/int", get(random_integers))
        .route("/device/info", get(device_info))
        .with_state(state)
}

/// Root endpoint
async fn root() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "service": "Quantis QRNG API",
        "version": "1.0.0",
        "endpoints": [
            "/api/v1/health",
            "/api/v1/random/bytes",
            "/api/v1/random/int",
            "/api/v1/device/info"
        ]
    }))
}

/// Health check endpoint
async fn health(State(state): State<AppState>) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut device = state.device.lock().await;
    
    match device.health_check() {
        Ok(true) => Ok(Json(serde_json::json!({
            "status": "healthy",
            "device": "connected",
            "buffer_available": state.buffer.available()
        }))),
        Ok(false) => Err(StatusCode::SERVICE_UNAVAILABLE),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

/// Generate random bytes
async fn random_bytes(
    Query(params): Query<BytesQuery>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<BytesResponse>>, StatusCode> {
    // Validate parameters
    if params.count == 0 || params.count > 65536 {
        return Ok(Json(ApiResponse::error("Count must be between 1 and 65536")));
    }

    // Try buffer first
    let raw_bytes = if let Some(bytes) = state.buffer.read(params.count) {
        bytes
    } else {
        // Fall back to direct device read
        let mut device = state.device.lock().await;
        match device.read(params.count) {
            Ok(bytes) => bytes,
            Err(e) => return Ok(Json(ApiResponse::error(format!("Device error: {}", e)))),
        }
    };

    // Apply bias correction
    let corrected_bytes = match params.correction.as_str() {
        "none" => bias_correction::none(&raw_bytes),
        "von_neumann" => {
            let corrected = bias_correction::von_neumann(&raw_bytes);
            if corrected.len() < params.count {
                // Need more raw data for von_neumann
                return Ok(Json(ApiResponse::error(
                    "Insufficient entropy after von_neumann correction, try larger count"
                )));
            }
            corrected
        }
        _ => return Ok(Json(ApiResponse::error("Invalid correction method"))),
    };

    // Format output
    let formatted = match params.format.as_str() {
        "hex" => hex::encode(&corrected_bytes[..params.count]),
        "base64" => base64::encode(&corrected_bytes[..params.count]),
        _ => return Ok(Json(ApiResponse::error("Invalid format"))),
    };

    Ok(Json(ApiResponse::success(BytesResponse {
        bytes: formatted,
        count: params.count,
        format: params.format,
        correction: params.correction,
    })))
}

/// Generate random integers
async fn random_integers(
    Query(params): Query<IntegersQuery>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<IntegersResponse>>, StatusCode> {
    // Validate parameters
    if params.min >= params.max {
        return Ok(Json(ApiResponse::error("min must be less than max")));
    }
    if params.count == 0 || params.count > 1000 {
        return Ok(Json(ApiResponse::error("count must be between 1 and 1000")));
    }

    let range = (params.max - params.min + 1) as u64;
    let bytes_per_int = ((range as f64).ln() / 256f64.ln()).ceil() as usize;
    let total_bytes = bytes_per_int * params.count * 2; // Extra for rejection sampling

    // Get random bytes
    let raw_bytes = if let Some(bytes) = state.buffer.read(total_bytes) {
        bytes
    } else {
        let mut device = state.device.lock().await;
        match device.read(total_bytes) {
            Ok(bytes) => bytes,
            Err(e) => return Ok(Json(ApiResponse::error(format!("Device error: {}", e)))),
        }
    };

    // Generate integers using rejection sampling
    let mut integers = Vec::with_capacity(params.count);
    let mut byte_offset = 0;

    while integers.len() < params.count && byte_offset + bytes_per_int <= raw_bytes.len() {
        let mut value = 0u64;
        for i in 0..bytes_per_int {
            value = (value << 8) | raw_bytes[byte_offset + i] as u64;
        }

        // Rejection sampling for uniform distribution
        let max_valid = u64::MAX - (u64::MAX % range);
        if value < max_valid {
            integers.push(params.min + (value % range) as i64);
        }

        byte_offset += bytes_per_int;
    }

    if integers.len() < params.count {
        return Ok(Json(ApiResponse::error("Insufficient entropy for requested integers")));
    }

    Ok(Json(ApiResponse::success(IntegersResponse {
        integers: integers.into_iter().take(params.count).collect(),
        min: params.min,
        max: params.max,
        count: params.count,
    })))
}

/// Get device information
async fn device_info(State(state): State<AppState>) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let mut device = state.device.lock().await;
    
    match device.info() {
        Ok(info) => Ok(Json(ApiResponse::success(serde_json::json!({
            "device": info,
            "buffer_size": state.buffer.capacity(),
            "buffer_available": state.buffer.available(),
        })))),
        Err(e) => Ok(Json(ApiResponse::error(format!("Failed to get device info: {}", e)))),
    }
}