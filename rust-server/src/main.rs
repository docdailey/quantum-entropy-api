//! Quantis QRNG Server
//!
//! High-performance REST API server for quantum random number generation
//! using ID Quantique Quantis hardware.

use anyhow::Result;
use axum::{Router, Server};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod api;
mod device;
mod utils;

use crate::device::QuantisDevice;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting Quantis QRNG Server v1.0.0");

    // Open Quantis device
    let device = match QuantisDevice::open(0) {
        Ok(dev) => {
            info!("Successfully opened Quantis device");
            Arc::new(Mutex::new(dev))
        }
        Err(e) => {
            eprintln!("Failed to open Quantis device: {}", e);
            eprintln!("Make sure the device is connected and you have permissions");
            eprintln!("You may need to run: sudo usermod -a -G plugdev $USER");
            std::process::exit(1);
        }
    };

    // Get device info
    {
        let mut dev = device.lock().await;
        match dev.info() {
            Ok(info) => {
                info!("Device: {}", info.product);
                info!("Serial: {}", info.serial);
                info!("Version: {}", info.version);
            }
            Err(e) => {
                eprintln!("Failed to get device info: {}", e);
            }
        }
    }

    // Create entropy buffer
    let buffer = Arc::new(utils::RingBuffer::new(16 * 1024 * 1024)); // 16MB buffer
    
    // Start background entropy reader
    utils::start_entropy_reader(device.clone(), buffer.clone()).await?;

    // Build router
    let app = Router::new()
        .nest("/api/v1", api::routes(device.clone(), buffer.clone()))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http());

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Listening on {}", addr);
    
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}