use crate::framework::walker::NodeRef;
use crate::syntax::ast::{CompilationUnit, TopLevelDeclaration};
use crate::syntax::nodes::declarations::{
    namespace_declaration::NamespaceBodyDeclaration, ClassBodyDeclaration, ClassDeclaration,
    MethodDeclaration, NamespaceDeclaration,
};
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::statements::statement::Statement;

/// Provides a generic way to enumerate children for any supported node.
pub trait Children<'a> {
    type Iter: Iterator<Item = NodeRef<'a>>;
    fn children(&'a self) -> Self::Iter;
}

/// Provides typed extraction from a NodeRef to a concrete AST type.
pub trait Extract<'a, T> {
    fn as_ref(&self) -> Option<&'a T>;
}

impl<'a> Children<'a> for NodeRef<'a> {
    type Iter = std::vec::IntoIter<NodeRef<'a>>;
    fn children(&'a self) -> Self::Iter { node_children(self).into_iter() }
}

fn node_children<'a>(node: &NodeRef<'a>) -> Vec<NodeRef<'a>> {
    match *node {
        NodeRef::CompilationUnit(cu) => {
            let mut out: Vec<NodeRef<'a>> = Vec::new();
            if let Some(fs) = &cu.file_scoped_namespace {
                for m in &fs.declarations {
                    match m {
                        NamespaceBodyDeclaration::Namespace(ns) => out.push(NodeRef::Namespace(ns)),
                        NamespaceBodyDeclaration::Class(c) => out.push(NodeRef::Class(c)),
                        NamespaceBodyDeclaration::Struct(_)
                        | NamespaceBodyDeclaration::Interface(_)
                        | NamespaceBodyDeclaration::Enum(_)
                        | NamespaceBodyDeclaration::Delegate(_)
                        | NamespaceBodyDeclaration::Record(_)
                        | NamespaceBodyDeclaration::GlobalAttribute(_) => {}
                    }
                }
            }
            for decl in &cu.declarations {
                match decl {
                    TopLevelDeclaration::Namespace(ns) => out.push(NodeRef::Namespace(ns)),
                    TopLevelDeclaration::Class(c) => out.push(NodeRef::Class(c)),
                    TopLevelDeclaration::Struct(_)
                    | TopLevelDeclaration::Record(_)
                    | TopLevelDeclaration::Interface(_)
                    | TopLevelDeclaration::Enum(_)
                    | TopLevelDeclaration::Delegate(_)
                    | TopLevelDeclaration::GlobalAttribute(_)
                    | TopLevelDeclaration::FileScopedNamespace(_) => {}
                }
            }
            for s in &cu.top_level_statements { out.push(NodeRef::Statement(s)); }
            out
        }
        NodeRef::Namespace(ns) => {
            let mut out: Vec<NodeRef<'a>> = Vec::new();
            for m in &ns.declarations {
                match m {
                    NamespaceBodyDeclaration::Namespace(inner) => out.push(NodeRef::Namespace(inner)),
                    NamespaceBodyDeclaration::Class(c) => out.push(NodeRef::Class(c)),
                    NamespaceBodyDeclaration::Struct(_)
                    | NamespaceBodyDeclaration::Interface(_)
                    | NamespaceBodyDeclaration::Enum(_)
                    | NamespaceBodyDeclaration::Delegate(_)
                    | NamespaceBodyDeclaration::Record(_)
                    | NamespaceBodyDeclaration::GlobalAttribute(_) => {}
                }
            }
            out
        }
        NodeRef::Class(class) => {
            let mut out: Vec<NodeRef<'a>> = Vec::new();
            for m in &class.body_declarations {
                match m {
                    ClassBodyDeclaration::Method(m) => out.push(NodeRef::Method(m)),
                    ClassBodyDeclaration::NestedClass(nested) => out.push(NodeRef::Class(nested)),
                    ClassBodyDeclaration::Constructor(c) => {
                        if let Some(body) = &c.body { out.push(NodeRef::Statement(body)); }
                    }
                    ClassBodyDeclaration::Record(_)
                    | ClassBodyDeclaration::Property(_)
                    | ClassBodyDeclaration::Field(_)
                    | ClassBodyDeclaration::Event(_)
                    | ClassBodyDeclaration::Indexer(_)
                    | ClassBodyDeclaration::Operator(_)
                    | ClassBodyDeclaration::Destructor(_)
                    | ClassBodyDeclaration::NestedStruct(_)
                    | ClassBodyDeclaration::NestedInterface(_)
                    | ClassBodyDeclaration::NestedEnum(_)
                    | ClassBodyDeclaration::NestedRecord(_) => {}
                }
            }
            out
        }
        NodeRef::Method(m) => {
            let mut out: Vec<NodeRef<'a>> = Vec::new();
            if let Some(body) = &m.body { out.push(NodeRef::Statement(body)); }
            out
        }
        NodeRef::Statement(stmt) => {
            use Statement::*;
            let mut out: Vec<NodeRef<'a>> = Vec::new();
            match stmt {
                If(s) => {
                    out.push(NodeRef::Statement(&s.consequence));
                    if let Some(alt) = &s.alternative { out.push(NodeRef::Statement(alt)); }
                }
                For(s) => { out.push(NodeRef::Statement(&s.body)); }
                ForEach(s) => { out.push(NodeRef::Statement(&s.body)); }
                While(s) => { out.push(NodeRef::Statement(&s.body)); }
                DoWhile(s) => { out.push(NodeRef::Statement(&s.body)); }
                Using(u) => { if let Some(b) = &u.body { out.push(NodeRef::Statement(b)); } }
                Switch(sw) => { for sec in &sw.sections { for s in &sec.statements { out.push(NodeRef::Statement(s)); } } }
                Try(t) => {
                    out.push(NodeRef::Statement(&t.try_block));
                    for c in &t.catches { out.push(NodeRef::Statement(&c.block)); }
                    if let Some(fin) = &t.finally_clause { out.push(NodeRef::Statement(&fin.block)); }
                }
                Block(stmts) => { for s in stmts { out.push(NodeRef::Statement(s)); } }
                Expression(e) => { out.push(NodeRef::Expression(e)); }
                _ => {}
            }
            out
        }
        NodeRef::Expression(_e) => {
            Vec::new()
        }
    }
}

impl<'a> Extract<'a, CompilationUnit> for NodeRef<'a> {
    fn as_ref(&self) -> Option<&'a CompilationUnit> {
        match *self { NodeRef::CompilationUnit(x) => Some(x), _ => None }
    }
}
impl<'a> Extract<'a, NamespaceDeclaration> for NodeRef<'a> {
    fn as_ref(&self) -> Option<&'a NamespaceDeclaration> {
        match *self { NodeRef::Namespace(x) => Some(x), _ => None }
    }
}
impl<'a> Extract<'a, ClassDeclaration> for NodeRef<'a> {
    fn as_ref(&self) -> Option<&'a ClassDeclaration> {
        match *self { NodeRef::Class(x) => Some(x), _ => None }
    }
}
impl<'a> Extract<'a, MethodDeclaration> for NodeRef<'a> {
    fn as_ref(&self) -> Option<&'a MethodDeclaration> {
        match *self { NodeRef::Method(x) => Some(x), _ => None }
    }
}
impl<'a> Extract<'a, Statement> for NodeRef<'a> {
    fn as_ref(&self) -> Option<&'a Statement> {
        match *self { NodeRef::Statement(x) => Some(x), _ => None }
    }
}
impl<'a> Extract<'a, Expression> for NodeRef<'a> {
    fn as_ref(&self) -> Option<&'a Expression> {
        match *self { NodeRef::Expression(x) => Some(x), _ => None }
    }
}

pub struct Query<'a> {
    start: NodeRef<'a>,
}

impl<'a> Query<'a> {
    pub fn from(start: NodeRef<'a>) -> Self { Self { start } }

    pub fn descendants(self) -> Descendants<'a> {
        let stack = node_children(&self.start);
        Descendants { stack }
    }

    pub fn of<T: 'a>(self) -> impl Iterator<Item = &'a T>
    where
        NodeRef<'a>: Extract<'a, T>,
    {
        // Collect to break borrows on per-iteration values cleanly
        let mut acc: Vec<&'a T> = Vec::new();
        for n in self.descendants() {
            if let Some(t) = <NodeRef<'a> as Extract<'a, T>>::as_ref(&n) {
                acc.push(t);
            }
        }
        acc.into_iter()
    }

    pub fn filter(self, p: impl Fn(&NodeRef<'a>) -> bool + 'a) -> impl Iterator<Item = NodeRef<'a>> + 'a {
        let v: Vec<NodeRef<'a>> = self.descendants().collect();
        v.into_iter().filter(move |n| p(n))
    }

    pub fn filter_typed<T: 'a>(self, p: impl Fn(&T) -> bool + 'a) -> impl Iterator<Item = &'a T> + 'a
    where
        NodeRef<'a>: Extract<'a, T>,
    {
        let v: Vec<&'a T> = self.of::<T>().collect();
        v.into_iter().filter(move |t| p(t))
    }
}

pub struct Descendants<'a> {
    stack: Vec<NodeRef<'a>>,
}

impl<'a> Iterator for Descendants<'a> {
    type Item = NodeRef<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let n = self.stack.pop()?;
        let children = node_children(&n);
        self.stack.extend(children);
        Some(n)
    }
}
