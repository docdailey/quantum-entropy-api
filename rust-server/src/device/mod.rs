//! Quantis device interface

use anyhow::Result;
use rusb::{Context, Device, DeviceHandle, UsbContext};
use serde::{Deserialize, Serialize};
use thiserror::Error;

const VENDOR_ID: u16 = 0x0aba;
const PRODUCT_ID: u16 = 0x0102;
const ENDPOINT_IN: u8 = 0x81;
const TIMEOUT_MS: u64 = 5000;

#[derive(Error, Debug)]
pub enum QuantisError {
    #[error("USB error: {0}")]
    Usb(#[from] rusb::Error),
    
    #[error("Device not found")]
    DeviceNotFound,
    
    #[error("Read timeout")]
    Timeout,
    
    #[error("Invalid response from device")]
    InvalidResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub product: String,
    pub serial: String,
    pub version: String,
}

pub struct QuantisDevice {
    handle: DeviceHandle<Context>,
    timeout: std::time::Duration,
}

impl QuantisDevice {
    /// Open a Quantis device by index
    pub fn open(index: usize) -> Result<Self, QuantisError> {
        let context = Context::new()?;
        
        // Find all Quantis devices
        let devices: Vec<Device<Context>> = context
            .devices()?
            .iter()
            .filter(|device| {
                if let Ok(desc) = device.device_descriptor() {
                    desc.vendor_id() == VENDOR_ID && desc.product_id() == PRODUCT_ID
                } else {
                    false
                }
            })
            .collect();
        
        if devices.is_empty() {
            return Err(QuantisError::DeviceNotFound);
        }
        
        if index >= devices.len() {
            return Err(QuantisError::DeviceNotFound);
        }
        
        let mut handle = devices[index].open()?;
        
        // Claim interface 0
        handle.claim_interface(0)?;
        
        Ok(Self {
            handle,
            timeout: std::time::Duration::from_millis(TIMEOUT_MS),
        })
    }
    
    /// Get device information
    pub fn info(&mut self) -> Result<DeviceInfo, QuantisError> {
        let device = self.handle.device();
        let desc = device.device_descriptor()?;
        
        let product = self.handle
            .read_product_string_ascii(&desc)
            .unwrap_or_else(|_| "Unknown".to_string());
            
        let serial = self.handle
            .read_serial_number_string_ascii(&desc)
            .unwrap_or_else(|_| "Unknown".to_string());
            
        Ok(DeviceInfo {
            product,
            serial,
            version: format!("{}.{}", desc.device_version().0, desc.device_version().1),
        })
    }
    
    /// Read raw entropy from the device
    pub fn read(&mut self, size: usize) -> Result<Vec<u8>, QuantisError> {
        let mut buffer = vec![0u8; size];
        let mut total_read = 0;
        
        while total_read < size {
            let chunk_size = (size - total_read).min(65536); // Max 64KB per transfer
            let bytes_read = self.handle.read_bulk(
                ENDPOINT_IN,
                &mut buffer[total_read..total_read + chunk_size],
                self.timeout,
            )?;
            
            if bytes_read == 0 {
                return Err(QuantisError::Timeout);
            }
            
            total_read += bytes_read;
        }
        
        Ok(buffer)
    }
    
    /// Check if device is healthy
    pub fn health_check(&mut self) -> Result<bool, QuantisError> {
        // Try to read a small amount of data
        match self.read(16) {
            Ok(data) => {
                // Basic entropy check - at least some variation
                let first = data[0];
                Ok(!data.iter().all(|&b| b == first))
            }
            Err(_) => Ok(false),
        }
    }
}

/// Bias correction algorithms
pub mod bias_correction {
    /// Von Neumann extractor - removes bias but reduces output by ~75%
    pub fn von_neumann(input: &[u8]) -> Vec<u8> {
        let mut output = Vec::with_capacity(input.len() / 4);
        let mut out_byte = 0u8;
        let mut out_bits = 0;
        
        for byte in input {
            for i in (0..8).step_by(2) {
                let bit1 = (byte >> i) & 1;
                let bit2 = (byte >> (i + 1)) & 1;
                
                match (bit1, bit2) {
                    (0, 1) => {
                        out_byte |= 0 << out_bits;
                        out_bits += 1;
                    }
                    (1, 0) => {
                        out_byte |= 1 << out_bits;
                        out_bits += 1;
                    }
                    _ => {} // Discard 00 and 11
                }
                
                if out_bits == 8 {
                    output.push(out_byte);
                    out_byte = 0;
                    out_bits = 0;
                }
            }
        }
        
        output
    }
    
    /// No correction - raw quantum data
    pub fn none(input: &[u8]) -> Vec<u8> {
        input.to_vec()
    }
}