use crate::errors::AppError;

pub fn validate_phone(phone: &str) -> Result<(), AppError> {
    if phone.len() != 11 || !phone.chars().all(|c| c.is_ascii_digit()) || !phone.starts_with('1') {
        return Err(AppError::BadRequest("Phone must be 11 digits starting with 1".into()));
    }
    Ok(())
}

pub fn validate_verify_code(code: &str) -> Result<(), AppError> {
    if code.len() != 6 || !code.chars().all(|c| c.is_ascii_digit()) {
        return Err(AppError::BadRequest("Verify code must be 6 digits".into()));
    }
    Ok(())
}

pub fn validate_string_max(value: &str, max_len: usize, field: &str) -> Result<(), AppError> {
    if value.trim().is_empty() {
        return Err(AppError::BadRequest(format!("{} must not be empty", field)));
    }
    if value.len() > max_len {
        return Err(AppError::BadRequest(format!("{} must be at most {} characters", field, max_len)));
    }
    Ok(())
}

pub fn validate_cake_size(size: &str) -> Result<(), AppError> {
    match size {
        "6inch" | "8inch" => Ok(()),
        _ => Err(AppError::BadRequest("cake_size must be '6inch' or '8inch'".into())),
    }
}

pub fn validate_cream_type(cream: &str) -> Result<(), AppError> {
    match cream {
        "animal" | "plant" => Ok(()),
        _ => Err(AppError::BadRequest("cream_type must be 'animal' or 'plant'".into())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_phones() {
        assert!(validate_phone("13800138000").is_ok());
        assert!(validate_phone("19999999999").is_ok());
    }

    #[test]
    fn invalid_phone_wrong_length() {
        assert!(validate_phone("1380013800").is_err());
        assert!(validate_phone("138001380000").is_err());
    }

    #[test]
    fn invalid_phone_non_digits() {
        assert!(validate_phone("1380013800a").is_err());
        assert!(validate_phone("1abcdefghij").is_err());
    }

    #[test]
    fn invalid_phone_wrong_start() {
        assert!(validate_phone("03800138000").is_err());
    }

    #[test]
    fn valid_verify_code() {
        assert!(validate_verify_code("123456").is_ok());
    }

    #[test]
    fn invalid_verify_code() {
        assert!(validate_verify_code("12345").is_err());
        assert!(validate_verify_code("12345a").is_err());
        assert!(validate_verify_code("1234567").is_err());
    }

    #[test]
    fn valid_string_max() {
        assert!(validate_string_max("hello", 10, "field").is_ok());
    }

    #[test]
    fn empty_string_rejected() {
        assert!(validate_string_max("   ", 10, "field").is_err());
    }

    #[test]
    fn too_long_string_rejected() {
        assert!(validate_string_max("abcdefghijk", 10, "field").is_err());
    }

    #[test]
    fn valid_cake_sizes() {
        assert!(validate_cake_size("6inch").is_ok());
        assert!(validate_cake_size("8inch").is_ok());
    }

    #[test]
    fn invalid_cake_size() {
        assert!(validate_cake_size("10inch").is_err());
        assert!(validate_cake_size("").is_err());
    }

    #[test]
    fn valid_cream_types() {
        assert!(validate_cream_type("animal").is_ok());
        assert!(validate_cream_type("plant").is_ok());
    }

    #[test]
    fn invalid_cream_type() {
        assert!(validate_cream_type("mixed").is_err());
    }
}
