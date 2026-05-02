use std::{fmt::Display, str::FromStr};

use serde::de::Error;
use serde::{Deserialize, Serialize};

use crate::ipi::validate::{IpiError, valid_ipi_base_number, valid_ipi_name_number};

mod validate;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash)]
pub struct IpiBaseNum {
    pub base: u64,
    pub remainder: u8,
    full: String,
}

impl IpiBaseNum {
    pub fn new(value: &str) -> Result<Self, IpiError> {
        let num = match valid_ipi_base_number(value) {
            Ok(x) => x,
            Err(e) => return Err(e),
        };
        let base = num - (num % 10);
        let rem = (num % 10) as u8;
        let full = format!("I-{base}-{rem}");

        Ok(Self {
            remainder: rem,
            base,
            full,
        })
    }
}

impl FromStr for IpiBaseNum {
    type Err = IpiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Eq, Hash)]
pub struct IpiNameNum(pub u64);
impl IpiNameNum {
    pub fn new(x: u64) -> Result<Self, IpiError> {
        match valid_ipi_name_number(x) {
            Ok(_) => Ok(IpiNameNum(x)),
            Err(e) => Err(e),
        }
    }
}

impl FromStr for IpiNameNum {
    type Err = IpiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digit = s
            .parse::<u64>()
            .map_err(|e| IpiError::BadFormat(e.to_string()))?;
        Self::new(digit)
    }
}

impl<'de> Deserialize<'de> for IpiNameNum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer)?;
        IpiNameNum::new(value)
            .map_err(|_| D::Error::custom(format!("Invalid IPI Name Number: {}", value)))
    }
}

impl Display for IpiNameNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>11.11}", self.0)?;
        Ok(())
    }
}

// --- SQLITE

#[cfg(feature = "sqlite")]
use sqlite::ReadableWithIndex;

#[cfg(feature = "sqlite")]
impl From<IpiBaseNum> for sqlite::Value {
    fn from(value: IpiBaseNum) -> Self {
        sqlite::Value::String(value.full)
    }
}

#[cfg(feature = "sqlite")]
impl ReadableWithIndex for IpiBaseNum {
    fn read<T: sqlite::ColumnIndex>(stmt: &sqlite::Statement, index: T) -> sqlite::Result<Self> {
        let Some(value) = stmt.read::<Option<String>, _>(index)? else {
            return Err(sqlite::Error {
                code: None,
                message: Some("IPI Base value is null / invalid".to_string()),
            });
        };
        match IpiBaseNum::new(&value) {
            Ok(s) => sqlite::Result::Ok(s),
            Err(_) => sqlite::Result::Err(sqlite::Error {
                code: None,
                message: Some(format!("invalid IPI base num: {}", value)),
            }),
        }
    }
}

#[cfg(feature = "sqlite")]
impl From<IpiNameNum> for sqlite::Value {
    fn from(value: IpiNameNum) -> Self {
        sqlite::Value::Integer(value.0 as i64)
    }
}

#[cfg(feature = "sqlite")]
impl ReadableWithIndex for IpiNameNum {
    fn read<T: sqlite::ColumnIndex>(stmt: &sqlite::Statement, index: T) -> sqlite::Result<Self> {
        let Some(digit) = stmt.read::<Option<i64>, _>(index)? else {
            return Err(sqlite::Error {
                code: None,
                message: Some("IPI value is null / invalid".to_string()),
            });
        };
        match IpiNameNum::new(digit as u64) {
            Ok(s) => sqlite::Result::Ok(s),
            Err(_) => sqlite::Result::Err(sqlite::Error {
                code: None,
                message: Some(format!("invalid IPI num: {}", digit)),
            }),
        }
    }
}
