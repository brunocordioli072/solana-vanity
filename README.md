# Solana Vanity

A CLI tool for generating Solana vanity addresses with custom prefixes.

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
⚡ Using 16 threads
Searching... 1250000 keys checked | 125000 keys/sec | Elapsed: 10.0s

🎉 Found a vanity address!
📍 Address: Sol7K9dqPPSh3udvYXQz4vvvJPPu8Mf8bxDmVvFqqr3
🔐 Private Key (Base58): 5J3mBbAH58CpQ3Y5RNJpUKPE62SQ5tfcvU2JpbnkeyhfsYB1Jcn...

📊 Performance Stats:
   Total keys checked: 1523847
   Time elapsed: 12.15s
   Average speed: 125411 keys/sec
```
## Performance

On modern hardware, expect speeds of 100,000+ keys/second.

| Processor                          | Threads/Cores     | Speed                |
|------------------------------------|-------------------|----------------------|
| Intel Core i9-13900F               | 32 threads        | ~1,400,000 keys/sec  |
| AMD EPYC 9K84                      | 369 threads       | ~23,000,000 keys/sec |
| AMD EPYC 9754                      | 492 threads       | ~26,000,000 keys/sec |

## License

MIT License - see [LICENSE](LICENSE) file for details
