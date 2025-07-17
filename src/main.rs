use clap::Parser;
use solana_vanity::{find_vanity_address};
use solana_sdk::signer::Signer;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The desired prefix for the vanity address (case-sensitive)
    #[arg(short, long)]
    prefix: Option<String>,

    /// Number of threads to use (defaults to available CPU cores)
    #[arg(short, long, default_value_t = num_cpus::get())]
    threads: usize,
    
}

fn main() {
    let args = Args::parse();

    match &args.prefix {
        Some(prefix) => {
            println!("🔍 Searching for Solana vanity address starting with: \"{}\"", prefix);
            println!("⚡ Using {} threads", args.threads);

            let result = find_vanity_address(prefix, args.threads);
            
            print_result(result);            
        },
    
        None => {
            eprintln!("❌ Error: Must specify either --prefix or --suffix");
            std::process::exit(1);
        }
    }
}

fn print_result(result: solana_vanity::VanityResult) {
    let pubkey_str = result.keypair.pubkey().to_string();
    let secret_key_bytes = result.keypair.to_bytes();
    let secret_key_base58 = bs58::encode(&secret_key_bytes).into_string();

    println!("\n\n🎉 Found a vanity address!");
    println!("📍 Address: {}", pubkey_str);
    println!("🔐 Private Key (Base58): {}", secret_key_base58);
    println!("\n📊 Performance Stats:");
    println!("   Total keys checked: {}", result.attempts);
    println!("   Time elapsed: {:.2}s", result.elapsed.as_secs_f64());
    println!("   Average speed: {:.0} keys/sec", result.attempts as f64 / result.elapsed.as_secs_f64());
    
    // Calculate estimated difficulty
    let difficulty = estimate_difficulty(&pubkey_str);
    if let Some(prob) = difficulty {
        println!("   Estimated difficulty: 1 in {:.0}", 1.0 / prob);
    }
}

fn estimate_difficulty(address: &str) -> Option<f64> {
    // Estimate probability based on Base58 alphabet (58 characters)
    // This is a rough estimate for prefixes/suffixes
    let base58_chars: f64 = 58.0;
    
    // Count consecutive characters from start (prefix)
    let prefix_len = address.chars()
        .take_while(|c| c.is_alphanumeric())
        .count();
    
    if prefix_len > 1 {
        Some(1.0 / base58_chars.powi(prefix_len as i32 - 1))
    } else {
        None
    }
}
