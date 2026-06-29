use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize, de};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum PublisherRole {
    #[default]
    OriginalPublisher,
    IncomeParticipant,
    Acquirer,
    Admin,
    Substituted,
    SubPublisher,
}

impl Display for PublisherRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            PublisherRole::OriginalPublisher => "E",
            PublisherRole::IncomeParticipant => "PA",
            PublisherRole::Acquirer => "AQ",
            PublisherRole::Admin => "AM",
            PublisherRole::Substituted => "ES",
            PublisherRole::SubPublisher => "SE",
        };
        write!(f, "{x}")
    }
}
impl FromStr for PublisherRole {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = match s {
            "E" => PublisherRole::OriginalPublisher,
            "PA" => PublisherRole::IncomeParticipant,
            "AQ" => PublisherRole::Acquirer,
            "AM" => PublisherRole::Admin,
            "ES" => PublisherRole::Substituted,
            "SE" => PublisherRole::SubPublisher,
            _ => return Err(()),
        };
        Ok(x)
    }
}

impl<'de> Deserialize<'de> for PublisherRole {
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
impl Serialize for PublisherRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
