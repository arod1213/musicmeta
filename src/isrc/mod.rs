use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize, de};

#[derive(Debug, Clone, PartialEq)]
pub struct Isrc {
    pub country: String,
    pub creator: String,
    pub year_suffix: u8,
    pub id: u32,
}

impl Serialize for Isrc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Isrc {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let text = String::deserialize(deserializer)?;
        match Isrc::from_str(&text) {
            Ok(x) => Ok(x),
            Err(_) => Err(de::Error::custom(format!("invalid isrc {text}"))),
        }
    }
}

impl Display for Isrc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:>2.2}-{:<3.3}-{:0>2.2}-{:>5.5}",
            self.country, self.creator, self.year_suffix, self.id
        )
    }
}

// CC-XXX-YY-NNNNN
// country, registrant, year, serial
fn clean_isrc(x: &str) -> String {
    x.replace(" ", "")
        .replace("-", "")
        .to_uppercase()
        .to_string()
}

impl FromStr for Isrc {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clean = clean_isrc(s);
        if clean.len() != 12 {
            return Err(());
        }
        let country = clean[0..2].to_string();
        let creator = clean[2..5].to_string();
        let year_suffix = clean[5..7].parse::<u8>().map_err(|_| ())?;
        let id = clean[7..].parse::<u32>().map_err(|_| ())?;
        Ok(Isrc {
            country,
            creator,
            year_suffix,
            id,
        })
    }
}
