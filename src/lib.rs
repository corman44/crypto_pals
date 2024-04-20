use base64::{Engine as _, engine::general_purpose};
use lazy_static::lazy_static;
use openssl::symm::{decrypt, encrypt, Cipher, Crypter, Mode};
use std::{collections::HashMap, u8};

// sourced from ol' reliable.. ChatGPT :D
lazy_static![
    static ref FREQENCY_MAP: HashMap<&'static str, f32> = [
        (" ", 17.0),
        ("E", 10.6),
        ("T", 7.7),
        ("A", 7.5),
        ("O", 7.1),
        ("N", 6.7),
        ("I", 6.6),
        ("S", 5.9),
        ("R", 5.9),
        ("H", 4.8),
        ("L", 4.0),
        ("D", 3.4),
        ("C", 3.1),
        ("U", 2.7),
        ("M", 2.5),
        ("F", 2.4),
        ("P", 2.0),
        ("G", 1.9),
        ("W", 1.7),
        ("Y", 1.5),
        ("B", 1.5),
        ("V", 1.0),
        ("K", 0.8),
        ("X", 0.2),
        ("J", 0.2),
        ("Q", 0.1),
        ("Z", 0.1)
        ].iter().copied().collect();
    ];

#[derive(Debug, Clone, Copy)]
pub struct ByteScore {
    pub byte: u8,
    pub score: f32,
}

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

#[inline]
pub fn single_byte_xor(mess: Vec<u8>, key: u8) -> Result<Vec<u8>, &'static str> {
    let output = mess.iter()
        .map(|x| {
            x ^ key
        })
        .collect::<Vec<u8>>();
    Ok(output)
}

#[inline]
pub fn repeating_key_xor(mess: Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>, &'static str> {
    let result = mess.iter()
        .enumerate()
        .map(|(idx, val)| {
            val ^ key[idx % key.len()]
        })
        .collect::<Vec<u8>>();
    Ok(result)
}

#[inline]
pub fn score_single_byte(bytes: Vec<u8>) -> Result<Vec<ByteScore>,&'static str> {
    // returns descending ordered Vec of ByteScores
    
    let mut scores = (0..=255).into_iter()
        .map(|key| {
            let temp_vec = single_byte_xor(bytes.clone(), key).unwrap();
            let score = temp_vec.iter()
                .map(|x|{
                    let mut key = *x as char;
                    if key >= (97 as char) && key <= (122 as char) {
                        key = ((key as u8) - 32) as char;
                    }
                    FREQENCY_MAP.get(key.to_string().as_str()).unwrap_or(&0.0)
                })
                .sum();

            ByteScore {
                byte: key,
                score: score
            }
        }).collect::<Vec<ByteScore>>();

    scores.sort_by(|v1, v2| v2.score.total_cmp(&v1.score));
    Ok(scores)
}

pub fn calc_hamming_distance(in1: Vec<u8>, in2: Vec<u8>) -> u32 {
    if in1.len() == in2.len() {
        let y = repeating_key_xor(in1, in2).expect("error with xor")
            .iter()
            .map(|x| x.count_ones())
            .sum::<u32>();
        y
    }
    else {
        0xFFFFFFFF
    }
}

pub fn splice_step(steps: u8, data: &Vec<u8>) -> Vec<Vec<u8>> {
    (0..steps).map(|offset| {
        data.iter()
            .copied()
            .skip(offset.into())
            .step_by(steps.into())
            .collect()
    }).collect()
}

#[inline]
pub fn pkcs7_pad(buffer: &mut Vec<u8>, bs: usize) {
    let rem = buffer.len() % bs;
    if rem > 0 {
        let temp = vec![bs as u8; rem];
        buffer.extend(temp.iter().cloned());
    }
    else {
        let temp = vec![bs as u8; bs];
        buffer.extend(temp.iter().cloned());
    }
}

#[inline]
pub fn decrypt_aes_128_ecb(input: Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>, &'static str> {
    let cipher = Cipher::aes_128_ecb();
    let decrypted = decrypt(cipher, &key, None, &input).unwrap();
    // let plaintext_size = input.len() + 16 - (input.len() % 16);
    // let mut decrypted = vec![0u8; plaintext_size];
    // let decrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Decrypt, &key, None);
    // let decrypted_size = decrypter.unwrap().update(&input, decrypted.as_mut_slice()).unwrap();
    // decrypted.resize(decrypted_size, 0);
    Ok(decrypted)
}

#[inline]
pub fn encrypt_aes_128_ecb(input: Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>, &'static str> {
    let cipher = Cipher::aes_128_ecb();
    let encrypted = encrypt(cipher, &key, None, &input).unwrap();
    // let mut encrypted = vec![0u8; input.len() + 16];
    // let encrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Encrypt, &key, None);
    // let encrypted_size = encrypter.unwrap().update(&input, encrypted.as_mut_slice()).unwrap();
    // println!("Encrypted resizing to: {}",encrypted_size);
    // encrypted.resize(encrypted_size, 0);
    Ok(encrypted)
}

// ----- Traits -----

pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for Vec<u8> {
    fn to_string(&self) -> String {
        self.iter().map(|x| {
            *x as char
        })
        .collect::<String>()
    }
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

impl ToBytes for String {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(self.len()/2);
        for nib in 0..(self.len()/2) {
            let res = u8::from_str_radix(&self[2*nib .. 2*nib+2], 16);
            match res {
                Ok(v) => bytes.push(v),
                Err(e) => println!("Problem with to_bytes: {}", e),
            };
        };
        bytes
    }
}
