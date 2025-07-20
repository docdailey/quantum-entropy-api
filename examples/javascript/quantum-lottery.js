#!/usr/bin/env node
/**
 * Quantum Lottery - Pick truly random lottery numbers
 * 
 * Supports multiple lottery formats:
 * - Powerball (5 + 1)
 * - Mega Millions (5 + 1)
 * - EuroMillions (5 + 2)
 * - Custom formats
 */

const QuantumClient = require('./quantum-client');

class QuantumLottery {
    constructor() {
        this.quantum = new QuantumClient();
        
        // Lottery configurations
        this.lotteries = {
            powerball: {
                name: 'Powerball',
                main: { count: 5, min: 1, max: 69 },
                bonus: { count: 1, min: 1, max: 26, name: 'Powerball' },
                emoji: '🔴'
            },
            megamillions: {
                name: 'Mega Millions',
                main: { count: 5, min: 1, max: 70 },
                bonus: { count: 1, min: 1, max: 25, name: 'Mega Ball' },
                emoji: '🟡'
            },
            euromillions: {
                name: 'EuroMillions',
                main: { count: 5, min: 1, max: 50 },
                bonus: { count: 2, min: 1, max: 12, name: 'Lucky Stars' },
                emoji: '⭐'
            },
            custom: {
                name: 'Custom Lottery',
                main: { count: 6, min: 1, max: 49 },
                emoji: '🎰'
            }
        };
    }

    /**
     * Pick unique random numbers
     */
    async pickNumbers(min, max, count) {
        // Get extra numbers to handle duplicates
        const response = await this.quantum.getIntegers(min, max, count * 2);
        
        // Get unique numbers
        const unique = new Set();
        for (const num of response) {
            unique.add(num);
            if (unique.size === count) break;
        }
        
        // If still need more, fetch additional
        while (unique.size < count) {
            const more = await this.quantum.getIntegers(min, max, count);
            for (const num of more) {
                unique.add(num);
                if (unique.size === count) break;
            }
        }
        
        return Array.from(unique).sort((a, b) => a - b);
    }

    /**
     * Generate lottery numbers for a specific game
     */
    async generate(lotteryType = 'powerball') {
        const config = this.lotteries[lotteryType];
        if (!config) {
            throw new Error(`Unknown lottery type: ${lotteryType}`);
        }

        console.log(`\n${config.emoji} ${config.name} Numbers ${config.emoji}`);
        console.log('═'.repeat(40));

        // Pick main numbers
        const mainNumbers = await this.pickNumbers(
            config.main.min,
            config.main.max,
            config.main.count
        );
        
        console.log(`Main numbers: ${mainNumbers.join(' - ')}`);

        // Pick bonus numbers if applicable
        if (config.bonus) {
            const bonusNumbers = await this.pickNumbers(
                config.bonus.min,
                config.bonus.max,
                config.bonus.count
            );
            
            const bonusLabel = config.bonus.name || 'Bonus';
            console.log(`${bonusLabel}: ${bonusNumbers.join(' - ')}`);
        }

        console.log('═'.repeat(40));
        console.log(`Generated: ${new Date().toLocaleString()}`);
        console.log('\nGood luck! 🍀\n');

        return { main: mainNumbers, bonus: config.bonus ? bonusNumbers : null };
    }

    /**
     * Generate multiple tickets
     */
    async generateMultiple(lotteryType, count) {
        console.log(`\n🎫 Generating ${count} ${this.lotteries[lotteryType].name} tickets...\n`);
        
        const tickets = [];
        for (let i = 1; i <= count; i++) {
            console.log(`Ticket #${i}:`);
            const numbers = await this.generate(lotteryType);
            tickets.push(numbers);
            console.log('');
        }
        
        return tickets;
    }

    /**
     * Interactive mode
     */
    async interactive() {
        const readline = require('readline');
        const rl = readline.createInterface({
            input: process.stdin,
            output: process.stdout
        });

        const question = (prompt) => new Promise(resolve => rl.question(prompt, resolve));

        console.log('\n🎰 Quantum Lottery Number Generator\n');
        console.log('Available lotteries:');
        Object.entries(this.lotteries).forEach(([key, config]) => {
            console.log(`  ${key}: ${config.name}`);
        });

        const type = await question('\nSelect lottery type (or press Enter for Powerball): ');
        const lotteryType = type.trim() || 'powerball';

        const countStr = await question('How many tickets? (default: 1): ');
        const count = parseInt(countStr) || 1;

        rl.close();

        if (count === 1) {
            await this.generate(lotteryType);
        } else {
            await this.generateMultiple(lotteryType, count);
        }
    }
}

// CLI interface
async function main() {
    const lottery = new QuantumLottery();
    const args = process.argv.slice(2);

    try {
        if (args.length === 0) {
            // Interactive mode
            await lottery.interactive();
        } else {
            const lotteryType = args[0].toLowerCase();
            const count = parseInt(args[1]) || 1;

            if (lotteryType === 'help') {
                console.log('\nUsage: quantum-lottery [type] [count]');
                console.log('\nTypes: powerball, megamillions, euromillions, custom');
                console.log('\nExamples:');
                console.log('  quantum-lottery                  # Interactive mode');
                console.log('  quantum-lottery powerball        # One Powerball ticket');
                console.log('  quantum-lottery megamillions 5   # Five Mega Millions tickets');
                return;
            }

            if (count === 1) {
                await lottery.generate(lotteryType);
            } else {
                await lottery.generateMultiple(lotteryType, count);
            }
        }
    } catch (error) {
        console.error('Error:', error.message);
        console.log('\nRun "quantum-lottery help" for usage information');
    }
}

// Export for use as module
module.exports = QuantumLottery;

// Run if called directly
if (require.main === module) {
    main();
}