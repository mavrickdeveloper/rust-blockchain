use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Define a simple hash function that simulates sequential hashing.
pub fn sequential_hash(input: &[u8]) -> Vec<u8> {
    let mut output = vec![0; input.len()];
    let mut state = 0;

    // Iterate through input bytes and create a sequential hash.
    for (i, &byte) in input.iter().enumerate() {
        state = state.wrapping_add(byte);
        output[i] = state;
    }

    output
}

// Define a structure for a PoH record, containing the hash and timestamp.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PoHRecord {
    pub hash: Vec<u8>,
    pub timestamp: u128,
}

// Define the Proof of History structure, containing a HashMap of records.
pub struct ProofOfHistory {
    pub records: HashMap<Vec<u8>, PoHRecord>,
}

// Implement methods for the Proof of History structure.
impl ProofOfHistory {
    // Create a new, empty Proof of History instance.
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
        }
    }

    // Add a new record to the Proof of History.
    pub fn record(&mut self, data: &[u8]) -> Vec<u8> {
        // Get the current time as a timestamp.
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let timestamp = current_time.as_micros();

        // Calculate the hash for the given data.
        let hash = sequential_hash(data);

        // Create a new PoHRecord with the calculated hash and timestamp.
        let poh_record = PoHRecord {
            hash: hash.clone(),
            timestamp,
        };

        // Insert the new record into the HashMap of records.
        self.records.insert(hash.clone(), poh_record);

        // Return the hash of the new record.
        hash
    }

    // Verify if a given record exists in the Proof of History and if it's valid.
    pub fn verify(&self, data: &[u8], hash: &[u8], timestamp: u128) -> bool {
        // Calculate the expected hash for the given data.
        let expected_hash = sequential_hash(data);

        // Check if the given hash exists in the records.
        if let Some(record) = self.records.get(hash) {
            // Compare the expected hash and given hash, as well as the timestamps.
            &expected_hash == hash && record.timestamp == timestamp
        } else {
            // If the hash is not in the records, return false.
            false
        }
    }
}

// The main function demonstrating the usage of the Proof of History structure.
fn main() {
    // Create a new Proof of History instance.
    let mut poh = ProofOfHistory::new();

    // Define the data to be recorded.
    let data = b"hello world";

    // Record the data in the Proof of History and obtain its hash.
    let hash = poh.record(data);

    // Retrieve the recorded data using the hash.
    let record = poh.records.get(&hash).unwrap();
    println!("Record: {:?}", record);

    // Verify the recorded data using the data, hash, and timestamp.
    let is_valid = poh.verify(data, &hash, record.timestamp);
    println!("Verification result: {}", is_valid);
}
