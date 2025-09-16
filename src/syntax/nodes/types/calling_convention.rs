use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum CallingConvention {
    Managed,
    Unmanaged,
}

impl CallingConvention {
    /// Parse a calling convention from a string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "managed" => Some(CallingConvention::Managed),
            "unmanaged" => Some(CallingConvention::Unmanaged),
            _ => None,
        }
    }
    
    /// Convert calling convention to string
    pub fn as_str(&self) -> &'static str {
        match self {
            CallingConvention::Managed => "managed",
            CallingConvention::Unmanaged => "unmanaged",
        }
    }
} 