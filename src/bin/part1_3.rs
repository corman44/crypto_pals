use std::collections::HashMap;
use crypto_pals::*;

fn main() {
    
}

fn single_byte_xor(mess: Vec<u8>, key: u8) -> Result<Vec<u8>, &'static str> {
    let output = mess.iter()
        .map(|x| {
            x ^ key
        })
        .collect::<Vec<u8>>();
    Ok(output)
}

fn vec_to_string(input: Vec<u8>) -> Result<String, &'static str> {
    Ok(input.iter().map(|x| {
        *x as char
    })
    .collect::<String>())
}



#[test]
fn set1_chal1() {
    let input = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    // sourced from ol' reliable.. ChatGPT :D
    let frequency_map: HashMap<&str, f32> = [
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
        ("Z", 0.1),
    ]
    .iter()
    .cloned()
    .collect();

    let bytes = string_to_bytes(input).unwrap();

    let mut scores = (0..=255).into_iter()
        .map(|key| {
            let mut score: f32 ;
            score = 0.0;
            //decode (XOR)
            let temp_vec = single_byte_xor(bytes.clone(), key).unwrap();

            //score it
            score = temp_vec.iter()
                .map(|x|{
                    // TODO: ensure value is upper case
                    let key = *x as char;
                    frequency_map.get(key.to_string().as_str()).unwrap_or(&0.0)
                })
                .sum();

            //provide score and key to return tuple
            (key, score)
        }).collect::<Vec<(u8,f32)>>();
    scores.sort_by(|(_, v1), (_, v2)| v2.total_cmp(v1));
    //println!("{:?}", scores);

    let high_score = scores[0].0;
    println!("High Score: {}",high_score);
    println!("Message from High Score:\n{}", 
        vec_to_string(single_byte_xor(
            bytes.clone(),
            high_score)
        .unwrap())
        .unwrap());
}