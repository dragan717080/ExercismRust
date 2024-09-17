use std::cmp::max;

use rand::Rng;

fn main() {
    let plain_text = "iamapandabear";
    let key = "abcdefghij";

    println!("{:?}", decode(key, &encode(key, plain_text).unwrap()));
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = rand::thread_rng();

    let mut key = String::new();
    let str_len = max(s.len(), 100);

    for _ in 0..str_len {
        key += &char::from_u32(97 + rng.gen_range(0..26)).unwrap().to_string();
    }

    let encoded_str = shift(&key, s, 1).unwrap();
    (key, encoded_str)
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    shift(key, s, 1)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    shift(key, s, -1)
}

fn shift(key: &str, s: &str, direction: i8) -> Option<String> {
    if key.is_empty() {
        return None;
    }

    let mut result = String::new();
    let mut i = 0;
    let mut key_arr = Vec::new();
    for c in key.chars() {
        if !c.is_ascii_lowercase() {
            return None;
        }
        key_arr.push(c);
    }
    for c in s.chars() {
        let shift = key_arr[i % key_arr.len()] as i8 - 'a' as i8;
        let n = ((c as i8 - b'a' as i8 + direction * shift) % 26 + 26) % 26;
        result.push(char::from(b'a' + n as u8));
        i += 1;
    }

    Some(result)
}
