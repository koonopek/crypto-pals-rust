use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use crate::bytes::{bytes_to_str, decode_hex, str_to_bytes, Bytes};

const ENG_FREQ_CHARS: [u8; 6] = [101, 116, 97, 105, 111, 110];
const ENG_FREQ_CHARS_WITH_SCORE: [(u8, u8); 6] =
    [(101, 12), (116, 9), (97, 8), (105, 7), (111, 6), (110, 6)];

fn xor(a: &Bytes, b: &Bytes) -> Bytes {
    a.iter()
        .zip(b.iter().cycle())
        .map(|(ai, bi)| ai ^ bi)
        .collect::<Vec<_>>()
}

fn hist_bytes(bytes: &Bytes) -> HashMap<u8, u64> {
    let mut hist: HashMap<u8, u64> = HashMap::new();
    for byte in bytes.iter() {
        let n = hist.entry(*byte).or_insert(0u64);
        *n += 1;
    }
    hist
}

#[test]
fn challange_2() {
    let input = decode_hex(str_to_bytes(String::from(
        "1c0111001f010100061a024b53535009181c",
    )));
    let key = decode_hex(str_to_bytes(String::from(
        "686974207468652062756c6c277320657965",
    )));
    let expected = decode_hex(str_to_bytes(String::from(
        "746865206b696420646f6e277420706c6179",
    )));

    assert_eq!(xor(&input, &key), expected);
}

#[test]
fn test_hist_bytes() {
    assert_eq!(hist_bytes(&vec![]), HashMap::new());
    assert_eq!(hist_bytes(&vec![0, 0]), HashMap::from([(0, 2)]));
    assert_eq!(
        hist_bytes(&vec![0, 1, 2, 3, 3, 1, 0]),
        HashMap::from([(0, 2), (1, 2), (2, 1), (3, 2)])
    );
}

fn get_possible_keys(letters: Vec<u8>, hist_bytes: &HashMap<u8, u64>, n: u8) -> Vec<(u8, u8, u64)> {
    let mut possible_keys = vec![];
    let mut sorted_hist: Vec<_> = hist_bytes.iter().collect();
    if sorted_hist.len() <= n.into() {
        panic!(
            "Can't get {} passwords from hist with {} points",
            n,
            sorted_hist.len()
        )
    }
    sorted_hist.sort_by(|a, b| a.1.cmp(b.1).reverse());

    for letter in letters {
        for i in 0..=n {
            let byte = sorted_hist.get(usize::from(i)).unwrap();
            possible_keys.push((byte.0 ^ letter, byte.0.to_owned(), byte.1.to_owned()));
        }
    }

    possible_keys
}

#[test]
fn test_get_possible_keys() {
    assert_eq!(
        get_possible_keys(
            ENG_FREQ_CHARS.to_vec(),
            &HashMap::from([(0, 1), (1, 0)]),
            0u8
        ),
        vec![
            (101, 0, 1),
            (116, 0, 1),
            (97, 0, 1),
            (105, 0, 1),
            (111, 0, 1),
            (110, 0, 1),
        ] // 5 letters x 1 combinations
    );
}

fn score_english_text(text: &Bytes) -> i64 {
    let scored_chars = HashMap::from(ENG_FREQ_CHARS_WITH_SCORE);

    let mut score: i64 = 0;
    for letter in text.iter() {
        match scored_chars.get(letter) {
            Some(letter_score) => score += i64::from(letter_score.to_owned()),
            None => score -= 10,
        }
    }

    score
}

#[test]
fn test_score_english_text() {
    assert_eq!(score_english_text(&str_to_bytes(String::from("ee"))), 24);
    assert_eq!(score_english_text(&str_to_bytes(String::from("0"))), -10);
    assert_eq!(score_english_text(&str_to_bytes(String::from("0e"))), 2);
    assert_eq!(score_english_text(&str_to_bytes(String::from("0eo"))), 8);
}

fn decrypt_xor_by_freq_letters(cipher_text: &Bytes) -> (String, i64) {
    let keys = get_possible_keys(ENG_FREQ_CHARS.to_vec(), &hist_bytes(&cipher_text), 1);

    let mut decrypted_score = vec![];
    for (key, enc_letter, n) in keys {
        let maybe_decrypted = xor(&cipher_text, &vec![key]);

        decrypted_score.push((
            bytes_to_str(&maybe_decrypted),
            score_english_text(&maybe_decrypted),
        ));
    }
    decrypted_score.sort_by(|a, b| a.1.cmp(&b.1).reverse());

    (String::from(&decrypted_score[0].0), decrypted_score[0].1)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[test]
fn challange_3() {
    let cipher_text = decode_hex(str_to_bytes(String::from(
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
    )));

    let decrypted = decrypt_xor_by_freq_letters(&cipher_text);

    assert_eq!("Cooking MC's like a pound of bacon", decrypted.0);
}

#[test]
fn challange_4() {
    let file_bytes = read_lines("./src/4.input.txt").unwrap();

    let mut best: String = String::new();
    let mut current_best_score = std::i64::MIN;

    for cipher in file_bytes {
        let decrypted = decrypt_xor_by_freq_letters(&decode_hex(str_to_bytes(cipher.unwrap())));
        if decrypted.1 > current_best_score {
            best = decrypted.0;
            current_best_score = decrypted.1;
        }
    }

    println!("result challenge 4: {}", best);
    assert_eq!(best, String::from("Now that the party is jumping\n"))
}

#[test]
fn challange_5() {
    let input = str_to_bytes(String::from(
        "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal",
    ));

    let expected = decode_hex(str_to_bytes(String::from("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f")));

    let encrypted = xor(&input, &str_to_bytes(String::from("ICE")));

    assert_eq!(expected.len(), encrypted.len());
    assert_eq!(expected, encrypted)
}
