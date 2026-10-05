use super::generate_private_key;

#[test]
fn generate_private_key_normalizes_pem_body() {
    let begin_marker = ["-----BEGIN RSA", " PRIVATE KEY-----"].concat();
    let end_marker = ["-----END RSA", " PRIVATE KEY-----"].concat();
    let raw = format!("{begin_marker}\\nMIIEpAIBAAKCAQEA7\n\nabc123+==\\n{end_marker}");

    let pem = generate_private_key(raw);

    assert!(pem.starts_with("-----BEGIN RSA PRIVATE KEY-----"));
    assert!(pem.ends_with("-----END RSA PRIVATE KEY-----\n"));
    assert!(pem.contains("abc123+=="));
    assert!(!pem.contains("\\n"));
    assert!(pem.contains('\n'));
}
