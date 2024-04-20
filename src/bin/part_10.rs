use std::fs::read_to_string;

use crypto_pals::*;

/*
Implement CBC mode
CBC mode is a block cipher mode that allows us to encrypt irregularly-sized messages, despite the fact that a block cipher natively only transforms individual blocks.
In CBC mode, each ciphertext block is added to the next plaintext block before the next call to the cipher core.
The first plaintext block, which has no associated previous ciphertext block, is added to a "fake 0th ciphertext block" called the initialization vector, or IV.
Implement CBC mode by hand by taking the ECB function you wrote earlier, making it encrypt instead of decrypt (verify this by decrypting whatever you encrypt to test), and using your XOR function from the previous exercise to combine them.
The file here is intelligible (somewhat) when CBC decrypted against "YELLOW SUBMARINE" with an IV of all ASCII 0 (\x00\x00\x00 &c)
Don't cheat.
Do not use OpenSSL's CBC code to do CBC mode, even to verify your results. What's the point of even doing this stuff if you aren't going to learn from it?
*/

fn main() {
    let filename = "inputs/input_10.txt";
    let lines: Vec<String> = read_to_string(filename).expect("unable to read from input file")
        .lines()
        .map(|line| line.to_string())
        .collect();

    let key = b"YELLOW SUBMARINE";
    let plaintext1 = b"hello world!\n Goodbye cruel world!!";
    println!("{:x?}", plaintext1);
    let _ = pkcs7_pad(&mut plaintext1.to_vec(), 16);
    println!("{:x?}", plaintext1);
    // TODO: chunk into blocksize chunks
    // TODO: define IV of all zeros
    // TODO: 
    encrypt_aes_128_ecb(vec![0u8; 40], vec![0u8;16]).unwrap();
}

#[cfg(test)]
mod tester {
    use super::*;

    #[test]
    fn test_encrypt_decrypte_cbc() {
        let key = Vec::from("YELLOW SUBMARINE");
        let pt = Vec::from("Hello world!\n Goodbye cruel world!!");

        let encrypted = encrypt_aes_128_ecb(pt.clone(), key.clone()).unwrap();
        let decrypted = decrypt_aes_128_ecb(encrypted, key).unwrap();
        assert_eq!(decrypted, pt);
    }
}