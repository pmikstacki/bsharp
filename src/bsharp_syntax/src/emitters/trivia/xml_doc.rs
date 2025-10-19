use crate::emitters::emit_trait::{Emit, EmitCtx, EmitError};
use crate::trivia::xml_documentation::{XmlAttribute, XmlElement, XmlNode, XmlDocumentationComment};

fn escape_xml(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(ch),
        }
    }
    out
}

impl Emit for XmlAttribute {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, _cx:&mut EmitCtx)->Result<(),EmitError>{
        write!(w, "{}=\"{}\"", self.name, escape_xml(&self.value))?;
        Ok(())
    }
}

impl Emit for XmlElement {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        // <name attr1="..." attr2="...">children</name>
        write!(w, "<{}", self.name)?;
        if !self.attributes.is_empty() {
            w.write_char(' ')?;
            for (i, a) in self.attributes.iter().enumerate() {
                if i != 0 { w.write_char(' ')?; }
                a.emit(w, cx)?;
            }
        }
        if self.children.is_empty() {
            w.write_str(" />")?;
            return Ok(())
        }
        w.write_char('>')?;
        for child in &self.children { child.emit(w, cx)?; }
        write!(w, "</{}>", self.name)?;
        Ok(())
    }
}

impl Emit for XmlNode {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        match self {
            XmlNode::Element(el) => el.emit(w, cx),
            XmlNode::Text(txt) => { w.write_str(&escape_xml(txt))?; Ok(()) }
            XmlNode::CData(c) => { write!(w, "<![CDATA[{}]]>", c)?; Ok(()) }
            XmlNode::Comment(c) => { write!(w, "<!--{}-->", c)?; Ok(()) }
        }
    }
}

impl Emit for XmlDocumentationComment {
    fn emit<W: std::fmt::Write>(&self, w:&mut W, cx:&mut EmitCtx)->Result<(),EmitError>{
        // Emit each top-level node on its own triple-slash line
        for (i, node) in self.elements.iter().enumerate() {
            if i != 0 { cx.nl(w)?; }
            cx.write_indent(w)?;
            w.write_str("/// ")?;
            node.emit(w, cx)?;
        }
        Ok(())
    }
}
