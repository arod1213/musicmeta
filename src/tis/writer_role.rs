use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize, de};

#[derive(Debug, Clone)]
pub enum WriterRole {
    Adaptor,
    Arranger,
    LyricAuthor,
    ComposerWriter,
    ComposerAuthor,
    SubArranger,
    SubAuthor,
    Translator,
    IncomeParticipant,
}
impl Default for WriterRole {
    fn default() -> Self {
        Self::ComposerAuthor
    }
}

impl FromStr for WriterRole {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = match s {
            "AD" => WriterRole::Adaptor,
            "AR" => WriterRole::Arranger,
            "A" => WriterRole::LyricAuthor,
            "C" => WriterRole::ComposerWriter,
            "CA" => WriterRole::ComposerAuthor,
            "SR" => WriterRole::SubArranger,
            "SA" => WriterRole::SubAuthor,
            "TR" => WriterRole::Translator,
            "PA" => WriterRole::IncomeParticipant,
            _ => return Err(()),
        };
        Ok(x)
    }
}

impl Display for WriterRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            WriterRole::Adaptor => "AD",
            WriterRole::Arranger => "AR",
            WriterRole::LyricAuthor => "A",
            WriterRole::ComposerWriter => "C",
            WriterRole::ComposerAuthor => "CA",
            WriterRole::SubArranger => "SR",
            WriterRole::SubAuthor => "SA",
            WriterRole::Translator => "TR",
            WriterRole::IncomeParticipant => "PA",
        };
        write!(f, "{x}")
    }
}

impl<'de> Deserialize<'de> for WriterRole {
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
impl Serialize for WriterRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
