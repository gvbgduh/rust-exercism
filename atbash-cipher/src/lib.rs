use std::str;

fn swap_the_letter(letter: char) -> Option<char> {
    match letter as u8 {
        b'a'..=b'z' => Some((b'a' + b'z' - letter as u8) as char),
        b'0'..=b'9' => Some(letter as char),
        _ => None
    }
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let l = plain.chars()
                 .filter_map(|letter| swap_the_letter(letter.to_ascii_lowercase()))
                 .collect::<String>();

    l.as_bytes()
        .chunks(5)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars()
          .filter_map(|letter| swap_the_letter(letter.to_ascii_lowercase()))
          .collect::<String>()
}
