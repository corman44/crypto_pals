use crypto_pals::*;

/*
Implement PKCS#7 padding
A block cipher transforms a fixed-sized block (usually 8 or 16 bytes) of plaintext into ciphertext. But we almost never want to transform a single block; we encrypt irregularly-sized messages.
One way we account for irregularly-sized messages is by padding, creating a plaintext that is an even multiple of the blocksize. The most popular padding scheme is called PKCS#7.
So: pad any block to a specific block length, by appending the number of bytes of padding to the end of the block. For instance,
"YELLOW SUBMARINE"
... padded to 20 bytes would be:
"YELLOW SUBMARINE\x04\x04\x04\x04"
 */

fn main() {
    todo!()
}

#[cfg(test)]
mod tester {
    use super::*;

    #[test]
    fn test_pkcs7_pad() {
        let mut input: Vec<u8> = Vec::from("YELLOW SUBMARINE");
        let mut expected: Vec<u8> = Vec::from("YELLOW SUBMARINE");
        (0..4).into_iter().for_each(|_| expected.push(4u8));

        pkcs7_pad(&mut input, 4);

        println!("{:x?}", input);
        assert_eq!(input, expected);
    }
}