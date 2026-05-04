use thiserror::Error;

#[derive(Debug, Error)]
pub enum IswcError {
    #[error("Invalid Iswc")]
    Invalid,
}

pub fn valid_iswc_digit(x: u64) -> bool {
    // max 10 digits
    if x > 9999999999 {
        return false;
    }
    let digits = format!("{:10.10}", x);
    iswc_checksum(&digits)
}

pub fn valid_iswc(iswc: &str) -> bool {
    let cleaned = iswc
        .replace(" ", "")
        .replace(".", "")
        .replace("-", "")
        .to_uppercase();

    if cleaned.len() != 11 {
        return false;
    }
    if !cleaned.starts_with("T") {
        return false;
    }
    let digits = &cleaned[1..];
    // if any non digit
    if digits.chars().into_iter().any(|c| !c.is_ascii_digit()) {
        return false;
    }
    dbg!(&digits);
    iswc_checksum(digits)
}

fn iswc_checksum(digits: &str) -> bool {
    let check_sum = digits.chars().take(9).enumerate().fold(1, |acc, (idx, c)| {
        let Some(digit) = c.to_digit(10) else {
            return acc;
        };
        acc + (idx as u32 + 1) * digit
    });
    let expected = (10 - (check_sum % 10)) % 10;
    let actual = digits.chars().nth(9).unwrap().to_digit(10).unwrap();
    expected == actual
}

#[cfg(test)]
mod test {
    use super::*;
    // T3221234234

    // #[test]
    // fn test_dmp() {
    // let x = "T3221234234";
    // assert!(valid_iswc(x));

    // let x = "T1234567893";
    // assert!(valid_iswc(x));
    // }

    #[test]
    fn test_seq() {
        let x = "T1234567894";
        assert!(valid_iswc(x));
    }

    #[test]
    fn test_drunk_tank() {
        let x = "T3237036583";
        assert!(valid_iswc(x));
    }
    #[test]
    fn test_peaches() {
        let x = "T-303.805.932-0";
        assert!(valid_iswc(x));
    }
}
