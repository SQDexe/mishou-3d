// /** Checks whether the provided string is a valid seven-character hexadecimal colour code. */
// pub fn check_hex_colour(value: &str) -> bool {
//     let correct_length = value.len() == 7;
//     let starts_with_hash = value.starts_with('#');
//     let is_hexadecimal = value.chars().skip(1).all(|c| c.is_ascii_hexdigit());

//     correct_length && starts_with_hash && is_hexadecimal
//     }

/** Checks whether the provided string consists exclusively of alphabetic characters, representing a named colour. */
pub fn check_named_colour(value: &str) -> bool {
    value.chars().all(|c| c.is_ascii_alphabetic())
    }