/// Compares two secrets in time that depends only on their length, so a caller cannot learn a shared
/// secret one byte at a time from response timing. Unequal lengths are unequal at once (the length of
/// the configured secret is not a secret worth the extra code).
pub fn constant_time_eq(left: &str, right: &str) -> bool {
    let (left, right) = (left.as_bytes(), right.as_bytes());
    if left.len() != right.len() {
        return false;
    }
    left.iter()
        .zip(right)
        .fold(0u8, |difference, (a, b)| difference | (a ^ b))
        == 0
}

#[cfg(test)]
#[path = "../tests/secret_unit_test.rs"]
mod tests;
