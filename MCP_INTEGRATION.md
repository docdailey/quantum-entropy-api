# MCP Integration for Quantum Entropy API

## 🤖 Using with Claude and AI Assistants

The Quantum Entropy API supports Model Context Protocol (MCP), allowing AI assistants like Claude to directly generate quantum random numbers.

### Installation

1. **Install the MCP server**:
```bash
npm install -g @quantum/mcp-server
```

2. **Configure Claude Desktop** (or your MCP client):

Edit your Claude configuration file:
```json
{
  "mcpServers": {
    "quantum-entropy": {
      "command": "npx",
      "args": ["@quantum/mcp-server"],
      "env": {
        "QUANTUM_API_KEY": "your_api_key_here",
        "QUANTUM_API_URL": "https://quantum.docdailey.ai"
      }
    }
  }
}
```

### Available MCP Functions

#### `generate_random_bytes`
Generate true quantum random bytes.
```typescript
{
  count: number,      // 1-1000 bytes
  format: string,     // "hex" | "base64" 
  correction: string  // "none" | "von_neumann" | "matrix"
}
```

#### `generate_random_integers`
Generate quantum random integers in a range.
```typescript
{
  min: number,
  max: number,
  count: number  // 1-100
}
```

#### `generate_cryptographic_key`
Generate secure cryptographic keys.
```typescript
{
  level: string  // "low" | "medium" | "high" | "quantum"
}
```

#### `generate_uuid`
Generate a UUID v4 using quantum randomness.

#### `generate_password`
Generate secure passwords with quantum entropy.
```typescript
{
  length: number,     // 8-128
  uppercase: boolean,
  lowercase: boolean,
  digits: boolean,
  symbols: boolean
}
```

#### `test_entropy_quality`
Test the quality of quantum entropy.
```typescript
{
  sample_size: number  // 1000-100000 bytes
}
```

### Example Usage in Claude

Once configured, you can ask Claude:

- "Generate a cryptographic key using quantum entropy"
- "Create a truly random password with 20 characters"
- "Generate 10 random numbers between 1 and 100 using quantum randomness"
- "Test the quality of the quantum entropy"

### Benefits of MCP Integration

1. **Direct Access**: No need to copy/paste API calls
2. **Type Safety**: Validated parameters
3. **Natural Language**: Just ask for what you need
4. **Integrated Results**: Results appear directly in conversation
5. **Secure**: API key stored in configuration, not in prompts

### Setting Up Your Own MCP Server

For developers who want to integrate the Quantum API into their own MCP servers:

```javascript
// Example MCP server implementation
import { Server } from '@modelcontextprotocol/sdk';
import axios from 'axios';

const server = new Server({
  name: "quantum-entropy",
  version: "1.0.0"
});

server.setRequestHandler('generate_random_bytes', async (params) => {
  const response = await axios.get(`${API_URL}/random/bytes`, {
    headers: { 'Authorization': `Bearer ${API_KEY}` },
    params: {
      count: params.count,
      format: params.format
    }
  });
  
  return response.data;
});

// Add more handlers...

server.start();
```

### Get Started

1. Sign up for an API key at https://quantum.docdailey.ai
2. Install the MCP server
3. Configure your AI assistant
4. Start generating true quantum randomness!

For support and questions, visit our [GitHub repository](https://github.com/docdailey/quantum-entropy-api).