use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GlobalUsingDirective {
    pub using_directive: super::UsingDirective, // Reuse the existing UsingDirective enum/struct
}
