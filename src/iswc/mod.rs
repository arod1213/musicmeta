mod validate;

use serde::de::Error;
use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::iswc::validate::{IswcError, valid_iswc, valid_iswc_digit};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Iswc(pub u64);
impl TryFrom<String> for Iswc {
    type Error = IswcError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match Self::new_from_str(&value) {
            Some(s) => Ok(s),
            None => Err(IswcError::Invalid),
        }
    }
}

impl Iswc {
    fn clean(x: &str) -> String {
        x.replace(" ", "")
            .replace(".", "")
            .replace("-", "")
            .to_uppercase()
            .to_string()
    }

    pub fn new(x: u64) -> Option<Self> {
        if !valid_iswc_digit(x) {
            return None;
        }
        Some(Self(x))
    }

    fn new_from_str(x: &str) -> Option<Self> {
        if !valid_iswc(x) {
            return None;
        }
        let clean = &Self::clean(x)[1..];
        let Ok(digits) = clean.parse::<u64>() else {
            return None;
        };
        Some(Self(digits))
    }
}

impl FromStr for Iswc {
    type Err = IswcError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::new_from_str(s) {
            Some(s) => Ok(s),
            None => Err(IswcError::Invalid),
        }
    }
}

impl Display for Iswc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "T{:0>10.10}", self.0)?;
        Ok(())
    }
}

impl Serialize for Iswc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Iswc {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Iswc::new_from_str(&value)
            .ok_or_else(|| D::Error::custom(format!("Invalid ISWC: {}", value)))
    }
}

#[cfg(test)]
mod test_iswc {
    use super::*;
    #[test]
    fn test_write() {
        let x = Iswc::new_from_str("T-303.805.932-0").unwrap();
        assert_eq!("T3038059320", x.to_string())
    }
}

// -- SQLITE
#[cfg(feature = "sqlite")]
use sqlite::ReadableWithIndex;

#[cfg(feature = "sqlite")]
impl From<Iswc> for sqlite::Value {
    fn from(value: Iswc) -> Self {
        sqlite::Value::Integer(value.0 as i64)
    }
}

#[cfg(feature = "sqlite")]
impl ReadableWithIndex for Iswc {
    fn read<T: sqlite::ColumnIndex>(stmt: &sqlite::Statement, index: T) -> sqlite::Result<Self> {
        let Some(value) = stmt.read::<Option<i64>, _>(index)? else {
            return Err(sqlite::Error {
                code: None,
                message: Some("IPI Base value is null / invalid".to_string()),
            });
        };
        match Iswc::new(value as u64) {
            Some(s) => sqlite::Result::Ok(s),
            None => sqlite::Result::Err(sqlite::Error {
                code: None,
                message: Some(format!("invalid IPI base num: {}", value)),
            }),
        }
    }
}
