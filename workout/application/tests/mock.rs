

#[tokio::test]
async fn main() {


}

#[cfg(test)]
mod tests {
    use base64::prelude::*;

    #[test]
    fn validate_decode() {
        let original = "Hello, world!";
        let encoded = BASE64_STANDARD.encode(original);
        let decoded = BASE64_STANDARD.decode(&encoded).unwrap();
        let decoded_str = String::from_utf8(decoded).unwrap();
        assert_eq!(original, decoded_str);
    }
}