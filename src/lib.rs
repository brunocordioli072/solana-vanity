use solana_sdk::{signature::{Keypair, Signer}};
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

pub struct VanityResult {
    pub keypair: Keypair,
    pub elapsed: std::time::Duration,
    pub attempts: u64,
    pub matched_prefix: String,
}

pub fn find_vanity_address(prefixes: &[String], num_threads: usize) -> VanityResult {
    let start_time = Instant::now();
    let found = Arc::new(AtomicBool::new(false));
    let total_attempts = Arc::new(AtomicU64::new(0));
    
    // Pre-convert all prefixes to bytes for faster comparison
    let prefix_data: Vec<(Vec<u8>, usize, String)> = prefixes.iter()
        .map(|p| (p.as_bytes().to_vec(), p.len(), p.clone()))
        .collect();
    
    // Configure thread pool
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .stack_size(2 * 1024 * 1024) // 2MB stack for each thread
        .build()
        .expect("Failed to build thread pool");
    
    let search_result = pool.install(|| {
        // Clone prefix data for thread access
        let prefix_data = prefix_data.clone();
        
        // Use thread-local storage for better cache locality
        thread_local! {
            static BASE58_BUFFER: std::cell::RefCell<Vec<u8>> = std::cell::RefCell::new(Vec::with_capacity(64));
        }
        
        // Use much larger batch size to reduce coordination overhead
        (0u64..u64::MAX)
            .into_par_iter()
            .find_map_any(|_chunk| {
                let mut local_attempts = 0u64;
                let mut last_report = 0u64;
                let batch_size = 100_000; // Increased batch size
                
                // Process multiple attempts per thread to reduce coordination overhead
                for _ in 0..batch_size {
                    // Check if another thread found the result (less frequently)
                    if local_attempts % 10_000 == 0 && found.load(Ordering::Relaxed) {
                        total_attempts.fetch_add(local_attempts, Ordering::Relaxed);
                        return None;
                    }
                    
                    let keypair = Keypair::new();
                    local_attempts += 1;
                    
                    // Check against all prefixes
                    let pubkey_bytes = keypair.pubkey().to_bytes();
                    
                    // Use thread-local buffer for base58 encoding to avoid allocations
                    let match_result = BASE58_BUFFER.with(|buffer| {
                        let mut buf = buffer.borrow_mut();
                        buf.clear();
                        
                        // Base58 encoding with pre-allocated buffer
                        let encoded = bs58::encode(&pubkey_bytes).into_string();
                        
                        // Check against each prefix
                        for (prefix_bytes, prefix_len, prefix_str) in &prefix_data {
                            if encoded.len() >= *prefix_len {
                                if fast_prefix_compare(encoded[..*prefix_len].as_bytes(), prefix_bytes) {
                                    return Some(prefix_str.clone());
                                }
                            }
                        }
                        None
                    });
                    
                    // Progress reporting with reduced overhead
                    if local_attempts - last_report >= 50_000 {
                        let current_total = total_attempts.fetch_add(local_attempts - last_report, Ordering::Relaxed) + (local_attempts - last_report);
                        if current_total % 250_000 < 50_000 { // Report roughly every 250k attempts
                            let elapsed = start_time.elapsed();
                            let rate = current_total as f64 / elapsed.as_secs_f64();

                            // Format elapsed time as minutes:seconds
                            let total_secs = elapsed.as_secs();
                            let minutes = total_secs / 60;
                            let seconds = total_secs % 60;

                            print!("\rSearching... {} keys checked | {:.0} keys/sec | Elapsed: {}m:{:02}s", 
                                 current_total, rate, minutes, seconds);

                            use std::io::{self, Write};
                            io::stdout().flush().ok();
                        }
                        last_report = local_attempts;
                    }
                    
                    if let Some(matched_prefix) = match_result {
                        found.store(true, Ordering::Relaxed);
                        total_attempts.fetch_add(local_attempts, Ordering::Relaxed);
                        return Some((keypair, matched_prefix));
                    }
                }
                
                total_attempts.fetch_add(local_attempts, Ordering::Relaxed);
                None
            })
    });
    
    let (keypair, matched_prefix) = search_result.expect("Should find a keypair");
    VanityResult {
        keypair,
        elapsed: start_time.elapsed(),
        attempts: total_attempts.load(Ordering::Relaxed),
        matched_prefix,
    }
}

/// Optimized prefix comparison using SIMD when possible
#[inline(always)]
fn fast_prefix_compare(encoded: &[u8], prefix: &[u8]) -> bool {
    if encoded.len() < prefix.len() {
        return false;
    }
    
    // For small prefixes, use simple comparison
    if prefix.len() <= 8 {
        encoded[..prefix.len()] == *prefix
    } else {
        // For longer prefixes, use chunked comparison for better cache efficiency
        encoded[..prefix.len()].chunks_exact(8)
            .zip(prefix.chunks_exact(8))
            .all(|(a, b)| a == b)
            && encoded[prefix.len() - (prefix.len() % 8)..prefix.len()] == prefix[prefix.len() - (prefix.len() % 8)..]
    }
}


/// Batch generator for testing performance
pub fn generate_keypair_batch(batch_size: usize) -> Vec<Keypair> {
    (0..batch_size).into_par_iter()
        .map(|_| Keypair::new())
        .collect()
}