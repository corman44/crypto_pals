use crypto_pals::hex_to_base64;

fn main() {
    
}

#[test]
pub fn set1_chal1 () {
    let input = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let expected_out = String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    assert_eq!(hex_to_base64(input).unwrap(),expected_out);
}
