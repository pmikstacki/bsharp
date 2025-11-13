use serde::{Deserialize, Serialize};
use std::str::FromStr;
use bsharp_diagnostics_macros::enum_as_str;

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum CallingConvention {
    Managed,
    Unmanaged,
}

enum_as_str!(CallingConvention { Managed => "managed", Unmanaged => "unmanaged" });

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
