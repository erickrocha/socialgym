use crate::domain::business_error::BusinessError;
use crate::domain::enums::{ImageType, MediaType};

pub fn handle_result<T>(
    result: Result<T, sea_orm::DbErr>,
    error_msg: &str,
) -> Result<T, BusinessError> {
    result.map_err(|e| {
        log::info!("{}: {:?}", error_msg, e.to_string());
        BusinessError::new(e.to_string())
    })
}

pub fn handle_option<T>(option: Option<T>, error_msg: &str) -> Result<T, BusinessError> {
    option.ok_or_else(|| {
        log::info!("{}", error_msg);
        BusinessError::new(error_msg.to_string())
    })
}

pub fn choose_image_type(image_type: &str) -> ImageType {
    match image_type {
        "avatar" => ImageType::Avatar,
        "cover" => ImageType::Cover,
        _ => ImageType::Avatar,
    }
}

pub fn choose_media_type(media_type: &str) -> MediaType {
    match media_type {
        "image" => MediaType::Image,
        "video" => MediaType::Video,
        "audio" => MediaType::Audio,
        _ => MediaType::Image,
    }
}

pub fn generate_private_key(raw_value: String) -> String {
    let processed = raw_value.replace("\\n", "\n");

    // 2. Filtra as linhas para separar o que é Header/Footer do que é Conteúdo
    let lines: Vec<&str> = processed
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    // 3. O "Recheio" (Base64): pegamos tudo que não é o BEGIN ou END
    // e limpamos qualquer caractere que não seja do alfabeto Base64
    let body: String = lines
        .iter()
        .filter(|line| !line.starts_with("-----"))
        .flat_map(|line| line.chars())
        .filter(|c| c.is_ascii_alphanumeric() || *c == '+' || *c == '/' || *c == '=')
        .collect();

    // 4. Montamos o PEM com a estrutura exata que o parser exige
    format!(
        "-----BEGIN RSA PRIVATE KEY-----\n{}\n-----END RSA PRIVATE KEY-----\n",
        body
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_private_key() {
        let raw = "-----BEGIN RSA PRIVATE KEY-----\\nMIIEpAIBAAKCAQEA7\n\nabc123+==\\n-----END RSA PRIVATE KEY-----".to_string();
        let pem = generate_private_key(raw);
        assert!(pem.starts_with("-----BEGIN RSA PRIVATE KEY-----"));
        assert!(pem.ends_with("-----END RSA PRIVATE KEY-----\n"));
        assert!(pem.contains("abc123+=="));
        // Should not contain escaped newlines
        assert!(!pem.contains("\\n"));
        // Should contain real newlines
        assert!(pem.contains("\n"));
    }
}
