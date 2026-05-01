use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Isrc {
    pub country: String,
    pub creator: String,
    pub year_suffix: u8,
    pub id: u32,
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
// impl Isrc {
//     fn clean(x: &str) -> String {
//         x.replace(" ", "")
//             .replace("-", "")
//             .to_uppercase()
//             .to_string()
//     }
// }
