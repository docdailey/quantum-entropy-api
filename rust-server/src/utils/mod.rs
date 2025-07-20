//! Utility modules

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::sync::Mutex;
use tracing::{error, info, warn};

use crate::device::QuantisDevice;

/// Lock-free ring buffer for entropy storage
pub struct RingBuffer {
    buffer: Vec<u8>,
    capacity: usize,
    read_pos: AtomicUsize,
    write_pos: AtomicUsize,
    available: AtomicUsize,
}

impl RingBuffer {
    /// Create new ring buffer with given capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![0u8; capacity],
            capacity,
            read_pos: AtomicUsize::new(0),
            write_pos: AtomicUsize::new(0),
            available: AtomicUsize::new(0),
        }
    }

    /// Get buffer capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get available bytes
    pub fn available(&self) -> usize {
        self.available.load(Ordering::Relaxed)
    }

    /// Write data to buffer
    pub fn write(&self, data: &[u8]) -> usize {
        let available = self.available.load(Ordering::Relaxed);
        let free_space = self.capacity - available;
        
        if free_space == 0 {
            return 0;
        }

        let to_write = data.len().min(free_space);
        let write_pos = self.write_pos.load(Ordering::Relaxed);

        // Handle wrap-around
        if write_pos + to_write > self.capacity {
            let first_part = self.capacity - write_pos;
            unsafe {
                std::ptr::copy_nonoverlapping(
                    data.as_ptr(),
                    self.buffer.as_ptr().add(write_pos) as *mut u8,
                    first_part,
                );
                std::ptr::copy_nonoverlapping(
                    data.as_ptr().add(first_part),
                    self.buffer.as_ptr() as *mut u8,
                    to_write - first_part,
                );
            }
            self.write_pos.store(to_write - first_part, Ordering::Relaxed);
        } else {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    data.as_ptr(),
                    self.buffer.as_ptr().add(write_pos) as *mut u8,
                    to_write,
                );
            }
            self.write_pos.store((write_pos + to_write) % self.capacity, Ordering::Relaxed);
        }

        self.available.fetch_add(to_write, Ordering::Relaxed);
        to_write
    }

    /// Read data from buffer
    pub fn read(&self, size: usize) -> Option<Vec<u8>> {
        let available = self.available.load(Ordering::Relaxed);
        
        if available < size {
            return None;
        }

        let mut output = vec![0u8; size];
        let read_pos = self.read_pos.load(Ordering::Relaxed);

        // Handle wrap-around
        if read_pos + size > self.capacity {
            let first_part = self.capacity - read_pos;
            unsafe {
                std::ptr::copy_nonoverlapping(
                    self.buffer.as_ptr().add(read_pos),
                    output.as_mut_ptr(),
                    first_part,
                );
                std::ptr::copy_nonoverlapping(
                    self.buffer.as_ptr(),
                    output.as_mut_ptr().add(first_part),
                    size - first_part,
                );
            }
            self.read_pos.store(size - first_part, Ordering::Relaxed);
        } else {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    self.buffer.as_ptr().add(read_pos),
                    output.as_mut_ptr(),
                    size,
                );
            }
            self.read_pos.store((read_pos + size) % self.capacity, Ordering::Relaxed);
        }

        self.available.fetch_sub(size, Ordering::Relaxed);
        Some(output)
    }
}

// Safety: RingBuffer uses atomics for synchronization
unsafe impl Send for RingBuffer {}
unsafe impl Sync for RingBuffer {}

/// Start background entropy reader
pub async fn start_entropy_reader(
    device: Arc<Mutex<QuantisDevice>>,
    buffer: Arc<RingBuffer>,
) -> anyhow::Result<()> {
    tokio::spawn(async move {
        info!("Starting entropy reader thread");
        let mut consecutive_errors = 0;
        
        loop {
            // Check buffer fill level
            let available = buffer.available();
            let capacity = buffer.capacity();
            let fill_percent = (available as f64 / capacity as f64) * 100.0;
            
            // Only read if buffer is less than 80% full
            if fill_percent < 80.0 {
                let read_size = ((capacity - available) / 2).min(65536);
                
                let mut device = device.lock().await;
                match device.read(read_size) {
                    Ok(data) => {
                        let written = buffer.write(&data);
                        if written < data.len() {
                            warn!("Buffer overflow, discarded {} bytes", data.len() - written);
                        }
                        consecutive_errors = 0;
                    }
                    Err(e) => {
                        error!("Failed to read from device: {}", e);
                        consecutive_errors += 1;
                        
                        if consecutive_errors > 10 {
                            error!("Too many consecutive errors, stopping entropy reader");
                            break;
                        }
                        
                        // Back off on errors
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    }
                }
            } else {
                // Buffer is full, wait a bit
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            }
        }
    });
    
    Ok(())
}