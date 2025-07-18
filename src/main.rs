use clap::Parser;
use solana_vanity::{find_vanity_address};
use solana_sdk::signer::Signer;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The desired prefixes for the vanity address (case-sensitive). Can specify multiple.
    #[arg(short, long)]
    prefix: Vec<String>,

    /// Number of threads to use (defaults to available CPU cores)
    #[arg(short, long, default_value_t = num_cpus::get())]
    threads: usize,
    
}

fn main() {
    let args = Args::parse();

    if args.prefix.is_empty() {
        eprintln!("❌ Error: Must specify at least one --prefix");
        std::process::exit(1);
    }

    println!("🔍 Searching for Solana vanity address starting with: {:?}", args.prefix);
    println!("⚡ Using {} threads", args.threads);

    let result = find_vanity_address(&args.prefix, args.threads);
    
    print_result(result);
}

fn print_result(result: solana_vanity::VanityResult) {
    let pubkey_str = result.keypair.pubkey().to_string();
    let secret_key_bytes = result.keypair.to_bytes();
    let secret_key_base58 = bs58::encode(&secret_key_bytes).into_string();

    write_match_to_file(&pubkey_str, &secret_key_base58);
    
    let total_secs = result.elapsed.as_secs();

    let minutes = total_secs / 60;
    let seconds = total_secs % 60;

    println!("\n\n🎉 Found a vanity address!");
    println!("📍 Address: {}", pubkey_str);
    println!("🎯 Matched prefix: \"{}\"", result.matched_prefix);
    println!("🔐 Private Key (Base58): {}", secret_key_base58);
    println!("\n📊 Performance Stats:");
    println!("   Total keys checked: {}", result.attempts);
    println!("   Time elapsed: {}m:{:02}s", minutes, seconds);
    println!("   Average speed: {:.0} keys/sec", result.attempts as f64 / total_secs as f64);
}

fn write_match_to_file(pubkey: &str, secret: &str) {
    use std::fs::OpenOptions;
    use std::io::Write;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("matches.txt")
        .unwrap();

    writeln!(file, "{} | {}", pubkey, secret).unwrap();
}