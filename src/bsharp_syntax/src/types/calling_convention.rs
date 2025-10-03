use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum CallingConvention {
    Managed,
    Unmanaged,
}

impl CallingConvention {
    /// Convert calling convention to string
    pub fn as_str(&self) -> &'static str {
        match self {
            CallingConvention::Managed => "managed",
            CallingConvention::Unmanaged => "unmanaged",
        }
    }
}

impl FromStr for CallingConvention {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "managed" => Ok(CallingConvention::Managed),
            "unmanaged" => Ok(CallingConvention::Unmanaged),
            _ => Err(()),
        }
    }
}
