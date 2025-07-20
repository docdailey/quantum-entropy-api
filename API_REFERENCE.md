# Quantum Entropy API Reference

## Base URL
```
https://quantum.docdailey.ai
```

## Authentication
All API endpoints require authentication via Bearer token:
```
Authorization: Bearer YOUR_API_KEY
```

## OpenAPI Documentation
Interactive API documentation available at: https://quantum.docdailey.ai/docs

## Core Endpoints

### Random Bytes
`GET /random/bytes`

Generate true quantum random bytes.

**Parameters:**
- `count` (integer): Number of bytes to generate
  - Range: 1-1024
  - Default: 32
- `format` (string): Output format
  - Values: `hex`, `base64`, `raw`
  - Default: `hex`

**Example Request:**
```bash
curl -H "Authorization: Bearer YOUR_API_KEY" \
  "https://quantum.docdailey.ai/random/bytes?count=64&format=hex"
```

**Example Response:**
```json
{
  "success": true,
  "data": {
    "bytes": "a3f2b1c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2",
    "count": 64,
    "format": "hex",
    "correction": "none"
  },
  "metadata": {
    "user_id": "your_user_id",
    "tier": "developer",
    "timestamp": "2024-01-15T10:30:45.123Z"
  }
}
```

### Random Integers
`GET /random/int`

Generate quantum random integers within a specified range.

**Parameters:**
- `min` (integer): Minimum value (inclusive)
  - Default: 0
- `max` (integer): Maximum value (inclusive)
  - Default: 100
- `count` (integer): Number of integers to generate
  - Range: 1-100
  - Default: 1

**Example Request:**
```bash
# Roll 5 dice
curl -H "Authorization: Bearer YOUR_API_KEY" \
  "https://quantum.docdailey.ai/random/int?min=1&max=6&count=5"
```

**Example Response:**
```json
{
  "success": true,
  "data": {
    "integers": [3, 1, 6, 4, 2],
    "min": 1,
    "max": 6,
    "count": 5
  },
  "metadata": {
    "user_id": "your_user_id",
    "tier": "developer",
    "timestamp": "2024-01-15T10:31:23.456Z"
  }
}
```

### Usage Statistics
`GET /usage`

Get your current API usage statistics.

**Example Request:**
```bash
curl -H "Authorization: Bearer YOUR_API_KEY" \
  "https://quantum.docdailey.ai/usage"
```

**Example Response:**
```json
{
  "user_id": "your_user_id",
  "tier": "developer",
  "usage": {
    "current_month": {
      "bytes": 524288,
      "requests": 156
    },
    "limits": {
      "monthly_mb": 100,
      "rate_limit": 10
    }
  }
}
```

## Rate Limits

| Tier | Monthly Data | Rate Limit |
|------|--------------|------------|
| Developer | 100 MB | 10 req/min |
| Professional | 1 GB | 60 req/min |
| Enterprise | 50 GB | 600 req/min |

## Error Responses

### 401 Unauthorized
```json
{
  "detail": "Invalid or missing API key"
}
```

### 402 Payment Required
```json
{
  "detail": "Monthly quota exceeded"
}
```

### 429 Too Many Requests
```json
{
  "detail": "Rate limit exceeded"
}
```

## SDKs and Libraries

### Python
```python
import requests

headers = {"Authorization": "Bearer YOUR_API_KEY"}
response = requests.get(
    "https://quantum.docdailey.ai/random/bytes?count=32",
    headers=headers
)
data = response.json()
```

### JavaScript
```javascript
const response = await fetch(
  'https://quantum.docdailey.ai/random/bytes?count=32',
  {
    headers: {
      'Authorization': 'Bearer YOUR_API_KEY'
    }
  }
);
const data = await response.json();
```

### cURL
```bash
curl -H "Authorization: Bearer YOUR_API_KEY" \
  "https://quantum.docdailey.ai/random/bytes?count=32"
```

## Advanced Features via MCP

For advanced cryptographic operations, use our MCP (Model Context Protocol) integration:
- Password generation with custom parameters
- Cryptographic key generation
- UUID v4 generation
- Entropy quality testing

See [MCP_INTEGRATION.md](MCP_INTEGRATION.md) for details.