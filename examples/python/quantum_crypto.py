#!/usr/bin/env python3
"""
Quantum Crypto - Cryptographic operations using quantum entropy

Features:
- Generate encryption keys
- Create secure passwords
- Generate UUIDs
- Create API tokens
"""

import requests
import json
import base64
from typing import Optional, Dict

API_BASE = "https://quantum-server.docdailey.ai"

class QuantumCrypto:
    """Cryptographic operations using quantum entropy"""
    
    def __init__(self, api_base: str = API_BASE):
        self.api_base = api_base
        self.session = requests.Session()
    
    def generate_key(self, bits: int = 256) -> Optional[Dict[str, any]]:
        """Generate a cryptographic key
        
        Args:
            bits: Key size (128, 192, 256, or 512 bits)
            
        Returns:
            Dictionary with key data or None on error
        """
        if bits not in [128, 192, 256, 512]:
            raise ValueError("Key size must be 128, 192, 256, or 512 bits")
        
        try:
            response = self.session.get(
                f"{self.api_base}/api/v1/crypto/key",
                params={"level": bits},
                timeout=5
            )
            response.raise_for_status()
            
            data = response.json()
            if data.get('success'):
                return data['data']
            else:
                print(f"Error: {data.get('error', 'Unknown error')}")
                return None
                
        except requests.exceptions.RequestException as e:
            print(f"Request error: {e}")
            return None
    
    def generate_password(self, 
                         length: int = 16,
                         uppercase: bool = True,
                         lowercase: bool = True,
                         digits: bool = True,
                         symbols: bool = False) -> Optional[str]:
        """Generate a secure password
        
        Args:
            length: Password length (8-128 characters)
            uppercase: Include uppercase letters
            lowercase: Include lowercase letters
            digits: Include numbers
            symbols: Include special characters
            
        Returns:
            Generated password or None on error
        """
        if not 8 <= length <= 128:
            raise ValueError("Password length must be between 8 and 128")
        
        try:
            response = self.session.get(
                f"{self.api_base}/api/v1/crypto/password",
                params={
                    "length": length,
                    "uppercase": uppercase,
                    "lowercase": lowercase,
                    "digits": digits,
                    "symbols": symbols
                },
                timeout=5
            )
            response.raise_for_status()
            
            data = response.json()
            if data.get('success'):
                return data['data']['password']
            else:
                print(f"Error: {data.get('error', 'Unknown error')}")
                return None
                
        except requests.exceptions.RequestException as e:
            print(f"Request error: {e}")
            return None
    
    def generate_uuid(self) -> Optional[str]:
        """Generate a UUID v4 using quantum randomness"""
        try:
            response = self.session.get(
                f"{self.api_base}/api/v1/crypto/uuid",
                timeout=5
            )
            response.raise_for_status()
            
            data = response.json()
            if data.get('success'):
                return data['data']['uuid']
            else:
                print(f"Error: {data.get('error', 'Unknown error')}")
                return None
                
        except requests.exceptions.RequestException as e:
            print(f"Request error: {e}")
            return None
    
    def generate_token(self, length: int = 32, url_safe: bool = True) -> Optional[str]:
        """Generate a secure token
        
        Args:
            length: Token length in bytes
            url_safe: Use URL-safe base64 encoding
            
        Returns:
            Generated token or None on error
        """
        try:
            response = self.session.get(
                f"{self.api_base}/api/v1/random/bytes",
                params={"count": length, "format": "base64"},
                timeout=5
            )
            response.raise_for_status()
            
            data = response.json()
            if data.get('success'):
                token = data['data']['bytes']
                if url_safe:
                    # Convert to URL-safe base64
                    token = token.replace('+', '-').replace('/', '_').rstrip('=')
                return token
            else:
                print(f"Error: {data.get('error', 'Unknown error')}")
                return None
                
        except requests.exceptions.RequestException as e:
            print(f"Request error: {e}")
            return None
    
    def calculate_entropy(self, length: int, charset_size: int) -> float:
        """Calculate password entropy in bits"""
        import math
        return math.log2(charset_size ** length)

def main():
    """CLI interface for quantum crypto operations"""
    import argparse
    
    parser = argparse.ArgumentParser(description="Quantum Crypto Tools")
    subparsers = parser.add_subparsers(dest='command', help='Commands')
    
    # Key generation
    key_parser = subparsers.add_parser('key', help='Generate encryption key')
    key_parser.add_argument('--bits', type=int, default=256, 
                           choices=[128, 192, 256, 512],
                           help='Key size in bits')
    
    # Password generation
    pass_parser = subparsers.add_parser('password', help='Generate password')
    pass_parser.add_argument('--length', type=int, default=16,
                            help='Password length (8-128)')
    pass_parser.add_argument('--symbols', action='store_true',
                            help='Include symbols')
    pass_parser.add_argument('--no-uppercase', action='store_true',
                            help='Exclude uppercase letters')
    pass_parser.add_argument('--no-lowercase', action='store_true',
                            help='Exclude lowercase letters')
    pass_parser.add_argument('--no-digits', action='store_true',
                            help='Exclude numbers')
    
    # UUID generation
    uuid_parser = subparsers.add_parser('uuid', help='Generate UUID')
    
    # Token generation
    token_parser = subparsers.add_parser('token', help='Generate token')
    token_parser.add_argument('--length', type=int, default=32,
                             help='Token length in bytes')
    
    args = parser.parse_args()
    
    qc = QuantumCrypto()
    
    if args.command == 'key':
        print(f"\n🔐 Generating {args.bits}-bit encryption key...")
        key_data = qc.generate_key(args.bits)
        if key_data:
            print(f"Key (hex): {key_data['key']}")
            print(f"Key (base64): {key_data['key_base64']}")
            print(f"Bits: {key_data['bits']}")
            print(f"Bytes: {key_data['bits'] // 8}")
    
    elif args.command == 'password':
        print(f"\n🔑 Generating {args.length}-character password...")
        password = qc.generate_password(
            length=args.length,
            uppercase=not args.no_uppercase,
            lowercase=not args.no_lowercase,
            digits=not args.no_digits,
            symbols=args.symbols
        )
        if password:
            print(f"Password: {password}")
            
            # Calculate entropy
            charset_size = 0
            if not args.no_uppercase: charset_size += 26
            if not args.no_lowercase: charset_size += 26
            if not args.no_digits: charset_size += 10
            if args.symbols: charset_size += 32
            
            entropy = qc.calculate_entropy(args.length, charset_size)
            print(f"Entropy: {entropy:.1f} bits")
            print(f"Charset size: {charset_size} characters")
    
    elif args.command == 'uuid':
        print("\n🆔 Generating quantum UUID...")
        uuid = qc.generate_uuid()
        if uuid:
            print(f"UUID: {uuid}")
    
    elif args.command == 'token':
        print(f"\n🎫 Generating {args.length}-byte token...")
        token = qc.generate_token(args.length)
        if token:
            print(f"Token: {token}")
            print(f"Length: {len(token)} characters")
    
    else:
        parser.print_help()

if __name__ == "__main__":
    main()