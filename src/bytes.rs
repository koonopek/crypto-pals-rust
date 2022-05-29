use std::borrow::Borrow;

pub type Bytes = Vec<u8>;

const BASE64_CHARS: [u8; 63] = [
    65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88,
    89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114,
    115, 116, 117, 118, 119, 120, 121, 122, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 43,
];

const ASCII_a: u8 = 97;
const ASCII_f: u8 = 102;

const ASCII_0: u8 = 48;
const ASCII_9: u8 = 57;

pub fn str_to_bytes(str: String) -> Bytes {
    str.into_bytes()
}

pub fn bytes_to_str(bytes: &Bytes) -> String {
    let string = std::str::from_utf8(bytes).unwrap_or("invalid utf-8");
    String::from(string)
}

fn bytes3_to_base64(a: &u8, b: &u8, c: &u8) -> [u8; 4] {
    let a6 = a & 252u8;
    let a2 = a & 3u8;
    let b4_1 = b & 240u8;
    let b4_2 = b & 15u8;
    let c2 = c & 192u8;
    let c6 = c & 63u8;

    [
        BASE64_CHARS[usize::from(a6 >> 2)],
        BASE64_CHARS[usize::from((a2 << 4) + (b4_1 >> 4))],
        BASE64_CHARS[usize::from((b4_2 << 2) + (c2 >> 6))],
        BASE64_CHARS[usize::from(c6)],
    ]
}

fn hex_to_base64(hex: &Bytes) -> Bytes {
    let mut iterator = hex.iter();
    let mut vec: Bytes = vec![];

    loop {
        match (iterator.next(), iterator.next(), iterator.next()) {
            (Some(a), Some(b), Some(c)) => vec.extend(bytes3_to_base64(a, b, c)),
            (Some(a), Some(b), None) => vec.extend(bytes3_to_base64(a, b, &0)),
            (Some(a), None, None) => vec.extend(bytes3_to_base64(a, &0, &0)),
            _ => break,
        }
    }

    vec
}

pub fn decode_hex(bytes: Bytes) -> Bytes {
    let mut iterator = bytes.iter();
    let mut vec: Bytes = vec![];
    let to_byte = |byte| match byte {
        n @ ASCII_0..=ASCII_9 => n - ASCII_0,
        n @ ASCII_a..=ASCII_f => (n - ASCII_a) + 10,
        n => panic!("Invalid hex character {}", n),
    };

    loop {
        match (iterator.next(), iterator.next()) {
            (Some(hi_half), Some(low_half)) => {
                vec.push((to_byte(*hi_half) << 4) + to_byte(*low_half))
            }
            (Some(hi_half), None) => vec.push(to_byte(*hi_half) << 4),
            _ => break,
        }
    }

    vec
}

#[test]
fn challange_1() {
    let input = decode_hex(
        str_to_bytes(String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"))
    );
    let expected = str_to_bytes(String::from(
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
    ));

    assert_eq!(hex_to_base64(input.borrow()).len(), expected.len());
}

#[test]
fn test_it_decode_hex() {
    assert_eq!(decode_hex(str_to_bytes(String::from("0f"))), [15]);
    assert_eq!(decode_hex(str_to_bytes(String::from("3e"))), [62]);
    assert_eq!(decode_hex(str_to_bytes(String::from("ff"))), [255]);
    assert_eq!(decode_hex(str_to_bytes(String::from("11"))), [17]);
    assert_eq!(decode_hex(str_to_bytes(String::from("ff3e"))), [255, 62]);
    assert_eq!(
        decode_hex(str_to_bytes(String::from("ff3eff3eff3e"))),
        [255, 62, 255, 62, 255, 62]
    );
}
