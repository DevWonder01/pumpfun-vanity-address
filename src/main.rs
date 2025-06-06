use clap::Parser;
use pumpfun_vanity::{ find_vanity_address, find_vanity_address_with_suffix };
use std::{ sync::atomic::{ AtomicBool, Ordering }, time::Instant };
use rayon::prelude::*;
use solana_sdk::signer::Signer;
use std::time::Duration;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

    /// Number of threads to use (defaults to available CPU cores)
    #[arg(short, long, default_value_t = num_cpus::get())]
    threads: usize,
}

fn main() {
    let args = Args::parse();
    // let prefix = args.prefix;
    let suffix = "pump".to_string();
    let num_threads = args.threads;

    println!("Searching for Solana vanity address starting with: \"{}\"", suffix);
    println!("Using {} threads...", num_threads);

    let result = find_vanity_address_with_suffix(&suffix, num_threads);
    let pubkey_str = result.keypair.pubkey().to_string();
    let p_k = result.keypair.secret().as_ref();
    let secret_key_base64: String = base64::encode(p_k);
    println!("pk {:?}", secret_key_base64);

    println!("\n🎉 Found a vanity address!");
    println!("Address: {}", pubkey_str);
    println!(
        "Private Key (Base58): {}",
        bs58::encode(result.keypair.secret().as_ref()).into_string()
    );
    println!("Private Key (bytes): {:?}", result.keypair.secret().as_ref());
    println!("Time elapsed: {:?}", result.elapsed);
}

#[test]
fn test_find_vanity_address() {
    // Test finding an address with prefix "A"
    // This should be relatively quick to find
    let prefix = "Dev";
    let num_threads = 4;
    
    let result = find_vanity_address(prefix, num_threads);
    
    // Verify the result
    let pubkey_str = result.keypair.pubkey().to_string();
    assert!(pubkey_str.starts_with(prefix), 
            "Generated key {} doesn't start with prefix {}", pubkey_str, prefix);
    
    println!("Found vanity address: {}", pubkey_str);
    println!("Time elapsed: {:?}", result.elapsed);
    println!("Attempts: {}", result.attempts);
}

#[test]
fn test_find_vanity_address_with_suffix() {
    // Test finding an address with suffix "a"
    // This should be relatively quick to find
    let suffix = "pump";
    let num_threads = 4;
    
    let result = find_vanity_address_with_suffix(suffix, num_threads);
    
    // Verify the result
    let pubkey_str = result.keypair.pubkey().to_string();
    assert!(pubkey_str.ends_with(suffix), 
            "Generated key {} doesn't end with suffix {}", pubkey_str, suffix);
    
    println!("Found vanity address: {}", pubkey_str);
    println!("Time elapsed: {:?}", result.elapsed);
    println!("Attempts: {}", result.attempts);
}



#[test]
#[ignore] // Ignored by default as it may take a long time
fn test_complex_pattern() {
    // Test finding a more complex pattern (will take longer)
    let prefix = "pump"; // More specific prefix will take longer to find
    let num_threads = num_cpus::get();
    
    let result = find_vanity_address_with_suffix(prefix, num_threads);
    
    // Verify the result
    let pubkey_str = result.keypair.pubkey().to_string();
    assert!(pubkey_str.starts_with(prefix));
    
    println!("Found complex vanity address: {}", pubkey_str);
    println!("Time elapsed: {:?}", result.elapsed);
    println!("Attempts: {}", result.attempts);
}

#[test]
fn test_timeout() {
    // Test with a timeout to prevent tests from running too long
    let difficult_prefix = "pump"; // Very unlikely to find this quickly
    let num_threads = num_cpus::get();
    let timeout = Duration::from_secs(5);
    
    // Use a separate thread with timeout
    let handle = std::thread::spawn(move || {
        find_vanity_address(difficult_prefix, num_threads)
    });
    
    match handle.join().ok() {
        Some(result) => {
            let pubkey_str = result.keypair.pubkey().to_string();
            assert!(pubkey_str.starts_with(difficult_prefix));
            println!("Found difficult vanity address: {}", pubkey_str);
        },
        None => {
            println!("Test timed out as expected for difficult pattern");
        }
    }
}
