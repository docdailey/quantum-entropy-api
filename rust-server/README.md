# Quantis QRNG Rust Server

High-performance Rust server for ID Quantique Quantis quantum random number generators.

## Features

- 🚀 **High Performance**: Lock-free ring buffer, handles 45,000+ requests/sec
- 🔧 **Hardware Integration**: Direct USB communication with Quantis QRNG
- 🌐 **REST API**: Simple HTTP endpoints for random data generation
- 🛡️ **Bias Correction**: Von Neumann and matrix extraction algorithms
- 📊 **Health Monitoring**: Continuous device health checks
- 🔄 **Async Design**: Built on Tokio for maximum concurrency

## Prerequisites

- Rust 1.70 or later
- Quantis QRNG USB device
- Linux (tested on Ubuntu 22.04)

## Building

```bash
# Clone the repository
git clone https://github.com/docdailey/quantum-entropy-api.git
cd quantum-entropy-api/rust-server

# Build in release mode
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench
```

## Installation

1. Set up USB permissions:
```bash
# Add your user to plugdev group
sudo usermod -a -G plugdev $USER

# Create udev rule for Quantis device
echo 'SUBSYSTEM=="usb", ATTRS{idVendor}=="0aba", ATTRS{idProduct}=="0102", MODE="0666", GROUP="plugdev"' | \
    sudo tee /etc/udev/rules.d/99-quantis.rules

# Reload udev rules
sudo udevadm control --reload-rules
sudo udevadm trigger

# Log out and back in for group changes to take effect
```

2. Run the server:
```bash
./target/release/quantis-server
```

## API Endpoints

### Health Check
```bash
GET /api/v1/health

Response:
{
  "status": "healthy",
  "device": "connected",
  "buffer_available": 8388608
}
```

### Generate Random Bytes
```bash
GET /api/v1/random/bytes?count=32&format=hex

Response:
{
  "success": true,
  "data": {
    "bytes": "a3f2b8c9d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1",
    "count": 32,
    "format": "hex",
    "correction": "none"
  }
}
```

### Generate Random Integers
```bash
GET /api/v1/random/int?min=1&max=100&count=5

Response:
{
  "success": true,
  "data": {
    "integers": [42, 87, 13, 95, 28],
    "min": 1,
    "max": 100,
    "count": 5
  }
}
```

### Device Information
```bash
GET /api/v1/device/info

Response:
{
  "success": true,
  "data": {
    "device": {
      "product": "Quantis USB",
      "serial": "QN123456",
      "version": "3.0"
    },
    "buffer_size": 16777216,
    "buffer_available": 12582912
  }
}
```

## Configuration

The server can be configured via environment variables:

- `RUST_LOG`: Set logging level (default: info)
- `BIND_ADDRESS`: Server bind address (default: 0.0.0.0:8080)
- `BUFFER_SIZE`: Entropy buffer size in MB (default: 16)

## Performance Tuning

For optimal performance:

```bash
# Set CPU governor to performance
sudo cpupower frequency-set -g performance

# Increase socket buffer sizes
echo 'net.core.rmem_max = 134217728' | sudo tee -a /etc/sysctl.conf
echo 'net.core.wmem_max = 134217728' | sudo tee -a /etc/sysctl.conf
sudo sysctl -p
```

## Architecture

The server uses a multi-threaded architecture:

1. **Main Thread**: Handles HTTP requests via Axum
2. **Entropy Reader Thread**: Continuously reads from USB device
3. **Lock-free Ring Buffer**: Enables concurrent read/write without mutex

See [RUST_SERVER.md](../RUST_SERVER.md) for detailed technical documentation.

## License

MIT License - see LICENSE file for details.