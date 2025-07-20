#!/usr/bin/env python3
"""
Quantum Dice - Roll dice using true quantum randomness

Examples:
    python quantum_dice.py          # Roll 2d6
    python quantum_dice.py 3d6      # Roll 3 six-sided dice
    python quantum_dice.py 1d20     # Roll a d20
    python quantum_dice.py 4d6 drop # Roll 4d6, drop lowest (D&D stats)
"""

import requests
import sys
from typing import List, Tuple

API_BASE = "https://quantum-server.docdailey.ai"

def roll_quantum_dice(num_dice: int = 2, sides: int = 6) -> List[int]:
    """Roll dice using quantum random numbers"""
    try:
        response = requests.get(
            f"{API_BASE}/api/v1/random/integers",
            params={"min": 1, "max": sides, "count": num_dice},
            timeout=5
        )
        response.raise_for_status()
        
        data = response.json()
        if data.get('success'):
            return data['data']
        else:
            raise Exception(data.get('error', 'Unknown error'))
            
    except requests.exceptions.RequestException as e:
        print(f"Error: {e}")
        return []

def parse_dice_notation(notation: str) -> Tuple[int, int]:
    """Parse dice notation like '3d6' into (count, sides)"""
    if 'd' not in notation:
        raise ValueError("Invalid dice notation. Use format: NdS (e.g., 3d6)")
    
    parts = notation.lower().split('d')
    num_dice = int(parts[0]) if parts[0] else 1
    sides = int(parts[1])
    
    if num_dice < 1 or num_dice > 100:
        raise ValueError("Number of dice must be between 1 and 100")
    if sides < 2 or sides > 1000:
        raise ValueError("Number of sides must be between 2 and 1000")
    
    return num_dice, sides

def roll_stats(dice_notation: str = "4d6") -> int:
    """Roll for D&D-style stats (4d6 drop lowest)"""
    num_dice, sides = parse_dice_notation(dice_notation)
    if num_dice < 4:
        print("Stats roll requires at least 4 dice")
        return 0
    
    rolls = roll_quantum_dice(num_dice, sides)
    if rolls:
        # Sort and drop lowest
        rolls.sort(reverse=True)
        kept_rolls = rolls[:3]
        dropped = rolls[3:]
        
        total = sum(kept_rolls)
        print(f"🎲 Rolled {dice_notation} drop lowest: {kept_rolls} (dropped {dropped})")
        print(f"   Stat total: {total}")
        return total
    return 0

def main():
    """Main CLI interface"""
    args = sys.argv[1:]
    
    if not args:
        # Default: roll 2d6
        rolls = roll_quantum_dice(2, 6)
        if rolls:
            print(f"🎲 Rolled 2d6: {rolls}")
            print(f"   Total: {sum(rolls)}")
    
    elif args[0] == "stats":
        # Roll 6 D&D stats
        print("🎯 Rolling D&D Stats (4d6 drop lowest):")
        print("-" * 30)
        stats = []
        for i in range(6):
            stat = roll_stats()
            stats.append(stat)
        print("-" * 30)
        print(f"Final stats: {stats}")
        print(f"Total: {sum(stats)}, Average: {sum(stats)/6:.1f}")
    
    elif "drop" in args:
        # Stats roll with custom dice
        dice_notation = args[0]
        roll_stats(dice_notation)
    
    else:
        # Parse dice notation
        try:
            num_dice, sides = parse_dice_notation(args[0])
            rolls = roll_quantum_dice(num_dice, sides)
            if rolls:
                print(f"🎲 Rolled {num_dice}d{sides}: {rolls}")
                print(f"   Total: {sum(rolls)}")
                
                # Show distribution for many dice
                if num_dice >= 10:
                    print(f"   Min: {min(rolls)}, Max: {max(rolls)}, Avg: {sum(rolls)/len(rolls):.1f}")
                    
        except ValueError as e:
            print(f"Error: {e}")
            print("Usage: quantum_dice.py [NdS] (e.g., 3d6, 1d20, 2d10)")

if __name__ == "__main__":
    main()