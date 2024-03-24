use crypto_pals::*;

fn main() {
    let input = String::from("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal");
    let key = String::from("ICE");
    let expected = String::from_utf8("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f".into()).unwrap();

    // println!("expected decrypted: {:?}", vec_to_string(repeating_key_xor(string_to_bytes(expected).unwrap(), key.as_bytes().to_vec()).unwrap()));
    println!("expected decrypted: {:?}", repeating_key_xor(expected.to_bytes(), key.as_bytes().to_vec()).unwrap().to_string());
}

#[test]
fn set1_chal5() {
    // back to test driving :D
    let input = String::from("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal");
    let key = String::from("ICE");
    let expected = String::from_utf8("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f".into()).unwrap();

    let bytes = input.as_bytes().to_vec();
    let key_bytes: Vec<u8> = key.as_bytes().to_vec();

    let encrypted_input = repeating_key_xor(bytes, key_bytes).unwrap();

    assert_eq!(encrypted_input, expected.to_bytes());
}