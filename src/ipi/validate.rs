#[derive(Debug, PartialEq)]
pub enum IpiError {
    ChecksumMismatch(u32, u32),
    BadFormat(String),
}

pub fn validate(
    num_str: &str,
    check_width: u32,
    modulus: u32,
    checksum: u32,
    base_val: u32,
    f: fn(u32, u32, u32) -> u32,
) -> Result<(), IpiError> {
    let sum = num_str.chars().enumerate().fold(base_val, |acc, (idx, c)| {
        f(acc, c.to_digit(10).unwrap(), idx as u32)
    });
    let remainder = sum % modulus;
    let expected = if remainder == 0 {
        0
    } else {
        (modulus - remainder) % check_width
    };
    if expected == checksum {
        Ok(())
    } else {
        Err(IpiError::ChecksumMismatch(expected, checksum))
    }
}

pub fn valid_ipi_name_number(num: u64) -> Result<(), IpiError> {
    if !(10..=99999999999).contains(&num) {
        return Err(IpiError::BadFormat(format!(
            "{num} is out of range: expected between 10..99999999999"
        )));
    }
    let num_str = format!("{:0>11.11}", num);
    let base = &num_str[0..9];
    let weight_fn = |acc: u32, x: u32, idx: u32| acc + x * (10 - idx);
    validate(&base, 100, 101, (num % 100) as u32, 0, weight_fn)
}

pub fn valid_ipi_base_number(value: &str) -> Result<u64, IpiError> {
    let clean = value.to_uppercase().replace("-", "");
    if !clean.starts_with("I") {
        return Err(IpiError::BadFormat(format!(
            "{value} does not start with 'I'"
        )));
    }
    let digits = &clean[1..];
    if digits.len() != 10 || !digits.chars().all(|c| c.is_ascii_digit()) {
        return Err(IpiError::BadFormat(format!(
            "{value} must match I-999999999-9"
        )));
    }

    let base = &digits[0..9];
    let base_num = base
        .parse::<u64>()
        .map_err(|e| IpiError::BadFormat(e.to_string()))?;

    let checksum = digits[9..10]
        .parse::<u32>()
        .map_err(|e| IpiError::BadFormat(e.to_string()))?;

    let weight_fn = |acc: u32, x: u32, idx: u32| acc + x * (idx + 1);
    match validate(base, 10, 10, checksum, 2, weight_fn) {
        Ok(_) => Ok(base_num),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_writer_ipi() {
        let x = 1051977450;
        assert_eq!(valid_ipi_name_number(x), Ok(()));
    }
    #[test]
    fn test_pub_ipi() {
        let x = 1051977352;
        assert_eq!(valid_ipi_name_number(x), Ok(()));
    }
    #[test]
    fn test_writer_ipi_base() {
        let x = "I-001068130-6";
        assert_eq!(valid_ipi_base_number(x), Ok(1068130));
    }
}
