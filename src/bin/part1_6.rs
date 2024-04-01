use std::fs::read_to_string;
use base64::{engine::general_purpose, prelude::*};
use crypto_pals::*;

/*
1.Let KEYSIZE be the guessed length of the key; try values from 2 to (say) 40.
2.Write a function to compute the edit distance/Hamming distance between two strings. The Hamming distance is just the number of differing bits. The distance between:
this is a test
and
wokka wokka!!!
is 37. Make sure your code agrees before you proceed.
3. For each KEYSIZE, take the first KEYSIZE worth of bytes, and the second KEYSIZE worth of bytes, and find the edit distance between them. Normalize this result by dividing by KEYSIZE.
4. The KEYSIZE with the smallest normalized edit distance is probably the key. You could proceed perhaps with the smallest 2-3 KEYSIZE values. Or take 4 KEYSIZE blocks instead of 2 and average the distances.
5. Now that you probably know the KEYSIZE: break the ciphertext into blocks of KEYSIZE length.
6. Now transpose the blocks: make a block that is the first byte of every block, and a block that is the second byte of every block, and so on.
7. Solve each block as if it was single-character XOR. You already have code to do this.
8. For each block, the single-byte XOR key that produces the best looking histogram is the repeating-key XOR key byte for that block. Put them together and you have the key.
 */

fn main() {
    let filename = "inputs/input_1_6.txt";
    let mut text = read_to_string(filename).expect("unable to read from input file");
    text.retain(|c| {
       !c.is_whitespace()
    });
    let decoded = general_purpose::STANDARD.decode(text).expect("unable to decode input");
    //println!("Decoded len: {}",decoded.len());

    // read first and second keysize bytes from 'decoded' and calculate hammingdistance
    let mut keysizes = (2..40).into_iter().map(|ks| {
        let hamming_calcs = calc_hamming_distance(
            decoded[0..ks].to_vec(),
            decoded[ks..ks*2].to_vec()
        )/ks as u32;
        (ks, hamming_calcs)
    }).collect::<Vec<(usize,u32)>>();
    keysizes.sort_by(|a,b| a.1.cmp(&b.1));
    // println!("decoded: {:?}",decoded);
    let prob_ks = keysizes[0].0;
    let splitup = splice_step(prob_ks as u8, &decoded);
    // println!("splitup: {:?}",splitup);
    //println!("Len of splitup: {}",splitup.len());
    // splitup.iter().for_each(|line|{
    //     println!("Line length: {}",line.len());
    // });
    let bs_list = splitup.iter().map(|bytes| {
        score_single_byte((*bytes.clone()).to_vec()).unwrap()
    })
    .collect::<Vec<Vec<ByteScore>>>();

    let key = bs_list.iter().map(|vbs| {
        vbs[0].byte
    })
    .collect::<Vec<u8>>();
    println!("key is: {:?}",key);
    println!("key is: {}",key.to_string());

    println!("decrypted:\n{}", repeating_key_xor(decoded, key).unwrap().to_string());

}

fn splice_step(steps: u8, data: &Vec<u8>) -> Vec<Vec<u8>> {
    (0..steps).map(|offset| {
        data.iter()
            .copied()
            .skip(offset.into())
            .step_by(steps.into())
            .collect()
    }).collect()
}

#[test]
fn set1_chal6() {
    let line1 = String::from("this is a test");
    let line2 = String::from("wokka wokka!!!");
    assert_eq!(calc_hamming_distance(line1.as_bytes().to_vec(), line2.as_bytes().to_vec()), 37);

    let steps: Vec<u8> = vec![1,2,3,4,5,6,7,8,9,10,11,12,13];
    let splitup = splice_step(3, &steps);
    assert_eq!(splitup[0], vec![1,4,7,10,13]);
    assert_eq!(splitup[1], vec![2,5,8,11]);
    assert_eq!(splitup[2], vec![3,6,9,12]);
}
