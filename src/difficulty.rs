pub const DIFFICULTY_PREFIX: &str = "00000000";

pub fn hash2binary(hash: &[u8]) -> String {
    let mut result = String::default();
    for byte in hash {
        result.push_str(&format!("{:08b}", byte));
    }
    result
}
