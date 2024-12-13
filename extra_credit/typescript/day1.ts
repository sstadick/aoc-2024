import * as fs from 'fs';
import * as readline from 'readline';

async function main() {
    const left: number[] = [];
    const right: number[] = [];

    const fileStream = fs.createReadStream('../../day-01/input1_bigger.txt');
    const rl = readline.createInterface({
        input: fileStream,
        crlfDelay: Infinity
    });

    try {
        for await (const line of rl) {
            const [a, b] = line.split(/\s+/).map(Number);
            left.push(a);
            right.push(b);
        }

        // Sort both arrays once
        const sortedLeft = left.sort((a, b) => a - b);
        const sortedRight = right.sort((a, b) => a - b);

        const result = sortedLeft
            .map((val, i) => Math.abs(val - sortedRight[i]))
            .reduce((sum, val) => sum + val, 0);

        console.log(result);
    } catch (error) {
        console.error('Error:', error);
    } finally {
        rl.close();
        fileStream.close();
    }
}

// Using an IIFE to handle the async main function
if (require.main === module) {
    (async () => {
        try {
            await main();
            process.exit(0);  // Explicitly exit after completion
        } catch (error) {
            console.error('Error:', error);
            process.exit(1);
        }
    })();
}