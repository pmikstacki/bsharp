// Assuming Identifier is in crate::nodes
use crate::Identifier;
use bsharp_syntax_derive::AstNode;
use serde::{Deserialize, Serialize};

/// Represents an attribute in an XML element (e.g., name="value").
#[derive(AstNode, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct XmlAttribute {
    pub name: Identifier, // e.g., "name" in <param name="value">
    pub value: String,    // e.g., "value" in <param name="value">
}

/// Represents an XML element within a documentation comment.
/// e.g., <summary>...</summary> or <param name="value">...</param>
#[derive(AstNode, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct XmlElement {
    pub name: Identifier, // e.g., "summary", "param"
    pub attributes: Vec<XmlAttribute>,
    pub children: Vec<XmlNode>, // Content of the element (text, other elements)
}

/// Represents a node within an XML structure, which can be an element, text, or CData.
#[derive(AstNode, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum XmlNode {
    Element(XmlElement),
    Text(String),    // Plain text content
    CData(String),   // CDATA section content
    Comment(String), // XML Comment <!-- ... -->
}

/// Represents a parsed C# XML documentation comment block.
/// Typically starts with /// or /**
#[derive(AstNode, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct XmlDocumentationComment {
    /// The root-level XML nodes found within the documentation comment.
    /// Common examples include <summary>, <remarks>, <param>, <returns>.
    pub elements: Vec<XmlNode>,
}

impl XmlElement {
    /// Helper method to find the value of a specific attribute by its name.
    pub fn get_attribute_value(&self, attribute_name: &str) -> Option<&String> {
        self.attributes
            .iter()
            .find(|attr| ident_equals(&attr.name, attribute_name))
            .map(|attr| &attr.value)
    }
}

impl XmlDocumentationComment {
    /// Helper method to find all top-level XML elements with a specific tag name.
    pub fn find_elements_by_name(&self, element_name: &str) -> Vec<&XmlElement> {
        self.elements
            .iter()
            .filter_map(|node| match node {
                XmlNode::Element(el) if ident_equals(&el.name, element_name) => Some(el),
                _ => None,
            })
            .collect()
    }
}

fn ident_equals(id: &Identifier, name: &str) -> bool {
    match id {
        Identifier::Simple(s) => s == name,
        Identifier::QualifiedIdentifier(segs) => segs.join(".") == name,
        Identifier::OperatorOverrideIdentifier(_) => false,
    }
}
