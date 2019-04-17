use std::str;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let l = plain.to_lowercase()
                 .chars()
                 .filter_map(|letter| match letter as u8 {
                    97..=122 => Some((97 + 122 - letter as u8) as char),
                    48..=57 => Some(letter as char),
                    _ => None
                 })
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
    cipher.to_lowercase()
        .chars()
        .filter_map(|letter| match letter as u8 {
        97..=122 => Some((97 + 122 - letter as u8) as char),
        48..=57 => Some(letter as char),
        _ => None
        })
        .collect::<String>()
}
