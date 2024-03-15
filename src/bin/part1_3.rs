use crypto_pals::*;

fn main() {
    let input = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    let bytes = string_to_bytes(input).unwrap();

    let scores = score_single_byte(bytes.clone()).unwrap();
    println!("{:?}", scores);

    let high_score = scores[0].byte;
    println!("High Score: {}",high_score);
    println!("Message from High Score:\n{}", 
        vec_to_string(single_byte_xor(
            bytes.clone(),
            high_score)
        .unwrap())
        .unwrap());
}

#[test]
fn set1_chal3() {
    // test derived from finding the answer in main XD
    let input = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    let bytes = string_to_bytes(input).unwrap();
    let scores = score_single_byte(bytes.clone()).unwrap();
    let high_score = scores[0].byte;

    assert_eq!(high_score, 88);
}