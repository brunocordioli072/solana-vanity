# Solana Vanity

*A CLI tool for generating Solana vanity addresses with custom prefixes. 2–3x faster than `solana-keygen grind`.*

## Installation

### From source

```bash
# Clone the repository
git clone https://github.com/brunocordioli072/solana-vanity.git
cd solana-vanity

# Build with optimizations
cargo build --release

# Run the binary
./target/release/solana-vanity --prefix ABC
```
## Usage

Generate a Solana address starting with a specific prefix:

```bash
# Run with release mode
cargo run --release -- --prefix ABC

# Run with multiple prefixes
cargo run --release -- --prefix AAA --prefix BBB --prefix CCC
```

## Example Output

```
🔍 Searching for Solana vanity address starting with: ["Sol"]
⚡ Using 32 threads
🚀 Searching... 3,000,000 keys checked | 1,500,000 keys/sec | Elapsed: 0m:02s

🎉 Found a vanity address!
🎯 Matched prefix: "Sol"
📍 Address: SolEzf1hwj6g8kqhUmHMBiAdRZjAmJX2TCevDKbfCNu
🔐 Private Key (Base58): 33a8EJBps5m1M3MTDo9MkTEAxujvdxL5JjFU...
💾 Saved to '/home/.../matches.txt'

📊 Performance Stats:
   Total keys checked: 3,000,000
   Time elapsed: 0m:02s
   Average speed: 1,500,000 keys/sec
```
## Performance

On modern hardware, expect speeds of 1,000,000+ keys/second.

| Processor                          | Threads/Cores     | Speed                |
|------------------------------------|-------------------|----------------------|
| Intel Core i9-13900F               | 32 threads        | ~1,400,000 keys/sec  |
| AMD EPYC 9K84                      | 369 threads       | ~23,000,000 keys/sec |
| AMD EPYC 9754                      | 492 threads       | ~26,000,000 keys/sec |

## License

MIT License - see [LICENSE](LICENSE) file for details
