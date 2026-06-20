use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize, de};

pub enum PublisherType {
    OriginalPublisher,
    IncomeParticipant,
    Acquirer,
    Admin,
    Substituted,
    SubPublisher,
}
impl Display for PublisherType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            PublisherType::OriginalPublisher => "E",
            PublisherType::IncomeParticipant => "PA",
            PublisherType::Acquirer => "AQ",
            PublisherType::Admin => "AM",
            PublisherType::Substituted => "ES",
            PublisherType::SubPublisher => "SE",
        };
        write!(f, "{x}")
    }
}
impl FromStr for PublisherType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = match s {
            "E" => PublisherType::OriginalPublisher,
            "PA" => PublisherType::IncomeParticipant,
            "AQ" => PublisherType::Acquirer,
            "AM" => PublisherType::Admin,
            "ES" => PublisherType::Substituted,
            "SE" => PublisherType::SubPublisher,
            _ => return Err(()),
        };
        Ok(x)
    }
}

impl<'de> Deserialize<'de> for PublisherType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let x = String::deserialize(deserializer)?;
        match Self::from_str(&x) {
            Ok(w) => Ok(w),
            Err(_) => Err(de::Error::custom("invalid designation")),
        }
    }
}
impl Serialize for PublisherType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
