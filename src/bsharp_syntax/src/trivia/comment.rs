use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommentKind {
    Line,
    Block,
    XmlDoc,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Comment {
    pub kind: CommentKind,
    pub text: String,
}
