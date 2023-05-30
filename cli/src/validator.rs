use cube::moves::scramble_from_string;

pub fn validate_scramble(scramble: &str) -> Result<String, String> {
    match scramble_from_string(&scramble) {
        Some(_) => Ok(scramble.to_owned()),
        None => Err("Invalid scramble".to_owned()),
    }
}
