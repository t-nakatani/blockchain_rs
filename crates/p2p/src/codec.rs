use anyhow::Result;
use serde::{Serialize, de::DeserializeOwned};
use bytes::Bytes;

// trait bounds: <T: Serialize>
pub fn encode<T: Serialize>(msg: &T) -> Result<Bytes> {
    Ok(Bytes::from(bincode::serialize(msg)?))
}

// trait bounds: <T: Deserialize>
pub fn decode<T: DeserializeOwned>(bytes: &Bytes) -> Result<T> {
    Ok(bincode::deserialize(bytes.as_ref())?)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let msg = "Hello, world!";  // &strはファットポインタで、メタデータとしてlenを持つ・格納先は静的メモリ領域・Stringは動的なのでヒープに格納される
        let encoded = encode(&msg).unwrap();
        let decoded = decode::<String>(&encoded).unwrap();
        assert_eq!(msg, decoded);
    }
}
