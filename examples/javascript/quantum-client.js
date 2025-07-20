#!/usr/bin/env node
/**
 * Quantum Entropy API Client for Node.js
 * 
 * Simple client library for accessing quantum random numbers
 */

const https = require('https');
const { URL } = require('url');

class QuantumClient {
    constructor(baseUrl = 'https://quantum-server.docdailey.ai') {
        this.baseUrl = baseUrl;
    }

    /**
     * Make API request
     * @private
     */
    async _request(endpoint, params = {}) {
        const url = new URL(`${this.baseUrl}${endpoint}`);
        Object.entries(params).forEach(([key, value]) => {
            url.searchParams.append(key, value);
        });

        return new Promise((resolve, reject) => {
            https.get(url.toString(), (res) => {
                let data = '';
                
                res.on('data', chunk => data += chunk);
                res.on('end', () => {
                    try {
                        const json = JSON.parse(data);
                        if (json.success) {
                            resolve(json.data);
                        } else {
                            reject(new Error(json.error || 'API error'));
                        }
                    } catch (e) {
                        reject(e);
                    }
                });
            }).on('error', reject);
        });
    }

    /**
     * Get random bytes
     * @param {number} count - Number of bytes (1-65536)
     * @param {string} format - Output format (hex, base64, raw)
     * @returns {Promise<Object>} Byte data
     */
    async getBytes(count = 32, format = 'hex') {
        return this._request('/api/v1/random/bytes', { count, format });
    }

    /**
     * Get random integers
     * @param {number} min - Minimum value
     * @param {number} max - Maximum value  
     * @param {number} count - Number of integers
     * @returns {Promise<Array>} Array of integers
     */
    async getIntegers(min, max, count = 1) {
        const data = await this._request('/api/v1/random/integers', { min, max, count });
        return data; // Returns array directly
    }

    /**
     * Generate password
     * @param {Object} options - Password options
     * @returns {Promise<string>} Generated password
     */
    async generatePassword(options = {}) {
        const params = {
            length: options.length || 16,
            uppercase: options.uppercase !== false,
            lowercase: options.lowercase !== false,
            digits: options.digits !== false,
            symbols: options.symbols || false
        };
        
        const data = await this._request('/api/v1/crypto/password', params);
        return data.password;
    }

    /**
     * Generate cryptographic key
     * @param {number} bits - Key size (128, 192, 256, 512)
     * @returns {Promise<Object>} Key data
     */
    async generateKey(bits = 256) {
        return this._request('/api/v1/crypto/key', { level: bits });
    }

    /**
     * Generate UUID
     * @returns {Promise<string>} UUID v4
     */
    async generateUUID() {
        const data = await this._request('/api/v1/crypto/uuid');
        return data.uuid;
    }

    /**
     * Stream quantum entropy (Node.js streams)
     * @param {Object} options - Stream options
     * @returns {Stream} Readable stream
     */
    streamEntropy(options = {}) {
        const { Readable } = require('stream');
        const url = new URL(`${this.baseUrl}/api/v1/stream/raw`);
        
        if (options.format) url.searchParams.append('format', options.format);
        if (options.correction) url.searchParams.append('correction', options.correction);

        const stream = new Readable({
            read() {}
        });

        https.get(url.toString(), (res) => {
            res.on('data', chunk => stream.push(chunk));
            res.on('end', () => stream.push(null));
            res.on('error', err => stream.destroy(err));
        });

        return stream;
    }
}

// Example usage and CLI
async function examples() {
    const quantum = new QuantumClient();

    console.log('🎲 Quantum Entropy Examples\n');

    try {
        // Random bytes
        const bytes = await quantum.getBytes(16);
        console.log('Random bytes (16):', bytes.bytes);

        // Random integers (dice roll)
        const dice = await quantum.getIntegers(1, 6, 2);
        console.log('Dice roll (2d6):', dice);

        // Secure password
        const password = await quantum.generatePassword({
            length: 20,
            symbols: true
        });
        console.log('Secure password:', password);

        // Encryption key
        const key = await quantum.generateKey(256);
        console.log('256-bit key:', key.key);

        // UUID
        const uuid = await quantum.generateUUID();
        console.log('UUID v4:', uuid);

        // Lottery numbers
        const lottery = await quantum.getIntegers(1, 49, 6);
        console.log('Lottery numbers:', lottery.sort((a, b) => a - b));

    } catch (error) {
        console.error('Error:', error.message);
    }
}

// Export for use as module
module.exports = QuantumClient;

// Run examples if called directly
if (require.main === module) {
    examples();
}