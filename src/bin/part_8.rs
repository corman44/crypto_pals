use std::{collections::HashSet, fs::read_to_string};

use crypto_pals::*;

/*
Detect AES in ECB mode
In this file are a bunch of hex-encoded ciphertexts.
One of them has been encrypted with ECB.
Detect it.
Remember that the problem with ECB is that it is stateless and deterministic; the same 16 byte plaintext block will always produce the same 16 byte ciphertext.
 */

fn main() {
    let filename = "inputs/input_1_8.txt";
    let lines: Vec<String> = read_to_string(filename).expect("unable to read from input file")
        .lines()
        .map(|line| line.to_string())
        .collect();
    let byte_lines = lines.iter().map(|line| line.to_bytes()).collect::<Vec<Vec<u8>>>();

    let mut unique = byte_lines.iter().filter(|line| !contains_duplicate(line.to_vec(), 16));
    let first_unique = unique.next().unwrap();
    println!("len: {}, {:x?}", first_unique.len(), first_unique);
}

pub fn contains_duplicate(ciphertext: Vec<u8>, chunksize: usize) -> bool {
    let split_buffer = ciphertext.chunks(chunksize);
    let mut unique = HashSet::new();
    !split_buffer.into_iter().all(|x| unique.insert(x))
}

#[cfg(test)]
mod tester {
    use super::*;

    #[test]
    fn test_contains_dup() {
        let text_dup = b"0123456789ABCDEF22222222222222220123456789ABCDEF";
        let text_nondup = b"0123456789ABCDEF22222222222222220123456783ABCDEF";
        assert!(contains_duplicate(text_dup.to_vec(), 16));
        assert!(!contains_duplicate(text_nondup.to_vec(), 16));
    }
}