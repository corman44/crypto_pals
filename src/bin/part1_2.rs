
fn main() {

}

fn xor_bytes(input1:  Vec<u8>, input2: Vec<u8>) -> Result<Vec<u8>, &'static str> {
    let output: Vec<u8> = input1.iter()
        .enumerate()
        .map(|(i, val)| {
            val ^ input2[i]        
        })
        .collect();
    Ok(output)
}

#[cfg(test)]
mod tester {
    use super::xor_bytes;
    use crypto_pals::string_to_bytes;

    #[test]
    fn xor_bytes_test() {
        let input1 = String::from("1c0111001f010100061a024b53535009181c");
        let input2 = String::from("686974207468652062756c6c277320657965");
        let expected = String::from("746865206b696420646f6e277420706c6179");
        assert_eq!(xor_bytes(string_to_bytes(input1).unwrap(), string_to_bytes(input2).unwrap()).unwrap(), string_to_bytes(expected).unwrap());
    }
}
