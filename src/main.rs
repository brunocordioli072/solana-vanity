use clap::Parser;
use solana_vanity::{find_vanity_address, format_number};
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

    
    let total_secs = result.elapsed.as_secs();

    let minutes = total_secs / 60;
    let seconds = total_secs % 60;

    println!("\n\n🎉 Found a vanity address!");
    println!("🎯 Matched prefix: \"{}\"", result.matched_prefix);
    println!("📍 Address: {}", pubkey_str);
    println!("🔐 Private Key (Base58): {}", secret_key_base58);
    write_match_to_file(&pubkey_str, &secret_key_base58);

    println!("\n📊 Performance Stats:");
    println!("   Total keys checked: {}", format_number(result.attempts));
    println!("   Time elapsed: {}m:{:02}s", minutes, seconds);
    println!("   Average speed: {} keys/sec", format_number(result.attempts / total_secs));
}

fn write_match_to_file(pubkey: &str, secret: &str) {
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::path::Path;

    let file_path = "matches.txt";

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .unwrap();

    writeln!(file, "{} | {}", pubkey, secret).unwrap();

    println!("💾 Saved to '{}'", Path::new(file_path).canonicalize().unwrap().display());
}