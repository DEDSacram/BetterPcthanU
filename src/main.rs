use rand::{RngCore, thread_rng};
use sha2::{Digest, Sha256};
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

fn main() {
    // Determine the number of threads to spawn based on CPU cores
    let num_threads = num_cpus::get();
    println!("Spawning {} threads for hashing", num_threads);

    // Shared variables
    let running = Arc::new(AtomicBool::new(true)); // Flag to stop hash threads
    let hash_count = Arc::new(AtomicUsize::new(0)); // Counter for completed hashes

    let mut handles = vec![];

    // Spawn hashing threads
    for _ in 0..num_threads {
        let running_hasher = Arc::clone(&running);
        let hash_count_hasher = Arc::clone(&hash_count);

        let handle = thread::spawn(move || {
            let mut rng = thread_rng();
            let mut input = [0u8; 10]; // 10-byte buffer for random input

            while running_hasher.load(Ordering::Relaxed) {
                rng.fill_bytes(&mut input); // Generate random 10-byte input
                let hash = sha256(&input); // Calculate the SHA-256 hash
                hash_count_hasher.fetch_add(1, Ordering::Relaxed); // Increment the counter
                let _ = hash; // Avoid unused variable warning
            }
        });

        handles.push(handle);
    }

    // Timer thread to stop all hashing threads after 5 seconds
    let running_timer = Arc::clone(&running);
    let timer = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1)); // Run for 5 seconds
        running_timer.store(false, Ordering::Relaxed); // Signal threads to stop
    });

    // Wait for the timer thread to complete
    timer.join().unwrap();

    // Wait for all hashing threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the total number of hashes completed
    println!(
        "Total hashes completed: {}",
        hash_count.load(Ordering::Relaxed)
    );
}

// Function to compute a SHA-256 hash
fn sha256(input: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize().to_vec()
}
