use crypto_pals::*;
use std::fs::read_to_string;

pub struct HighScoreTracker {
    idx: usize,
    bscore: ByteScore,
}

fn main() {
    let filename = "inputs/input_1_4.txt";
    let text_by_line = read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let mut high_scores = text_by_line.iter()
        .enumerate()
        .map(|(idx, line)| {
            let bytes = string_to_bytes(line.to_string()).unwrap();
            let scores = score_single_byte(bytes).unwrap();
            HighScoreTracker {
                idx: idx,
                bscore: scores[0]
            }
        })
        .collect::<Vec<HighScoreTracker>>();

    high_scores.sort_by(|v1,v2| v2.bscore.score.total_cmp(&v1.bscore.score));

    println!("Message from High Score:\n{}", 
        single_byte_xor(
            string_to_bytes(
                text_by_line[high_scores[0].idx]
                .to_string())
                .unwrap(),
            high_scores[0]
            .bscore.byte)
            .unwrap()
            .to_string());
    println!("Highscore Line: {}\nHighScore line byte: {}", high_scores[0].idx, high_scores[0].bscore.byte);
}

#[test]
fn set1_chal4() {
    // test driven is seeming more and more pointless... again derived from main..

    let filename = "inputs/input_1_4.txt";
    let text_by_line = read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();
    //println!("First Line:\n{}", text_by_line[0]);

    let mut high_scores = text_by_line.iter()
        .enumerate()
        .map(|(idx, line)| {
            let bytes = string_to_bytes(line.to_string()).unwrap();
            let scores = score_single_byte(bytes).unwrap();
            HighScoreTracker {
                idx: idx,
                bscore: scores[0]
            }
        })
        .collect::<Vec<HighScoreTracker>>();
    high_scores.sort_by(|v1,v2| v2.bscore.score.total_cmp(&v1.bscore.score));

    assert_eq!(high_scores[0].idx, 170);
    assert_eq!(high_scores[0].bscore.byte, 53);
}