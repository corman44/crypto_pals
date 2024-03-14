use std::u8;
use base64::{Engine as _, engine::general_purpose};

pub fn hex_to_base64(input: String) -> Result<String,&'static str> {
    let bytes = string_to_bytes(input).unwrap();
    let ret = general_purpose::STANDARD.encode(&bytes);
    Ok(ret)
}

pub fn string_to_bytes(input: String) -> Result<Vec<u8>, &'static str> {
    let mut bytes: Vec<u8> = Vec::with_capacity(input.len()/2);
    for nib in 0..(input.len()/2) {
        let res = u8::from_str_radix(&input[2*nib .. 2*nib+2], 16);
        match res {
            Ok(v) => bytes.push(v),
            Err(e) => println!("Problem with hex: {}", e),
        };
    };
    Ok(bytes)
}