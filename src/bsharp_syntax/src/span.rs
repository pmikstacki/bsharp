use serde::{Deserialize, Serialize};

pub type Span<'a> = nom_locate::LocatedSpan<&'a str>;