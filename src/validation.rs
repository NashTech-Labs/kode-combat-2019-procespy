pub static ERROR_MESSAGE: &str = "Unable to validate.";

pub fn validate_threshold(memory_consumed: &str, threshold: f64) -> Result<bool, &str> {
    match memory_consumed.parse::<f64>() {
        Ok(value) => {
            if value >= threshold {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(_) => Err(ERROR_MESSAGE),
    }
}

#[cfg(test)]
mod test {
    use crate::validation::validate_threshold;

    static MEMORY_USED: &str = "12.0";
    static MEMORY_CONSUMED: &str = "10.0";
    static INVALID_MEMORY_USED: &str = "test";
    static THRESHOLD: f64 = 11.0;

    #[test]
    fn test_validate_threshold_success() {
        assert!(validate_threshold(MEMORY_USED, THRESHOLD).is_ok());
    }

    #[test]
    fn test_validate_threshold_success_false() {
        assert!(validate_threshold(MEMORY_CONSUMED, THRESHOLD).is_ok());
    }

    #[test]
    fn test_validate_threshold_failure() {
        assert!(validate_threshold(INVALID_MEMORY_USED, THRESHOLD).is_err());
    }
}