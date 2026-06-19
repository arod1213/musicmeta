use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize, de};

#[derive(Debug)]
pub enum WriterDesignation {
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

impl FromStr for WriterDesignation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = match s {
            "AD" => WriterDesignation::Adaptor,
            "AR" => WriterDesignation::Arranger,
            "A" => WriterDesignation::LyricAuthor,
            "C" => WriterDesignation::ComposerWriter,
            "CA" => WriterDesignation::ComposerAuthor,
            "SR" => WriterDesignation::SubArranger,
            "SA" => WriterDesignation::SubAuthor,
            "TR" => WriterDesignation::Translator,
            "PA" => WriterDesignation::IncomeParticipant,
            _ => return Err(()),
        };
        Ok(x)
    }
}

impl Display for WriterDesignation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            WriterDesignation::Adaptor => "AD",
            WriterDesignation::Arranger => "AR",
            WriterDesignation::LyricAuthor => "A",
            WriterDesignation::ComposerWriter => "C",
            WriterDesignation::ComposerAuthor => "CA",
            WriterDesignation::SubArranger => "SR",
            WriterDesignation::SubAuthor => "SA",
            WriterDesignation::Translator => "TR",
            WriterDesignation::IncomeParticipant => "PA",
        };
        write!(f, "{x}")
    }
}

impl<'de> Deserialize<'de> for WriterDesignation {
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
impl Serialize for WriterDesignation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
