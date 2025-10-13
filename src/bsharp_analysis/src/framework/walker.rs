use crate::framework::NodeRef;
use crate::framework::visit::Visit;
use crate::framework::AnalysisSession;
use crate::syntax::ast::{CompilationUnit, TopLevelDeclaration};
use bsharp_syntax::declarations::{
    ClassBodyDeclaration, ClassDeclaration, MethodDeclaration, NamespaceBodyDeclaration,
    NamespaceDeclaration,
};
use bsharp_syntax::statements::statement::Statement;
use bsharp_syntax::statements::statement::Statement::{
    DoWhile, For, If, Switch, Try, Using, While,
};

impl Default for AstWalker<'_> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AstWalker<'a> {
    visitors: Vec<Box<dyn Visit + 'a>>, // All visitors are run in a single traversal
}

impl<'a> AstWalker<'a> {
    pub fn new() -> Self {
        Self {
            visitors: Vec::new(),
        }
    }

    pub fn with_visitor(mut self, v: Box<dyn Visit + 'a>) -> Self {
        self.visitors.push(v);
        self
    }

    pub fn run(&mut self, cu: &'a CompilationUnit, session: &mut AnalysisSession) {
        self.visit_compilation_unit(cu, session);
    }

    fn notify_enter(&mut self, node: &NodeRef, session: &mut AnalysisSession) {
        for v in self.visitors.iter_mut() {
            v.enter(node, session);
        }
    }

    fn notify_exit(&mut self, node: &NodeRef, session: &mut AnalysisSession) {
        for v in self.visitors.iter_mut().rev() {
            v.exit(node, session);
        }
    }

    fn visit_compilation_unit(&mut self, cu: &'a CompilationUnit, session: &mut AnalysisSession) {
        let node: NodeRef = NodeRef::from(cu);
        self.notify_enter(&node, session);

        // File-scoped namespace
        if let Some(fs) = &cu.file_scoped_namespace {
            // Wrap file-scoped namespace into a synthetic NamespaceDeclaration-like traversal by visiting its members
            for member in &fs.declarations {
                self.visit_namespace_member(member, session);
            }
        }

        // Top-level declarations
        for decl in &cu.declarations {
            match decl {
                TopLevelDeclaration::Namespace(ns) => self.visit_namespace(ns, session),
                TopLevelDeclaration::Class(class) => self.visit_class(class, session),
                TopLevelDeclaration::Struct(_)
                | TopLevelDeclaration::Record(_)
                | TopLevelDeclaration::Interface(_)
                | TopLevelDeclaration::Enum(_)
                | TopLevelDeclaration::Delegate(_)
                | TopLevelDeclaration::GlobalAttribute(_)
                | TopLevelDeclaration::FileScopedNamespace(_) => {}
            }
        }

        self.notify_exit(&node, session);
    }

    fn visit_namespace(&mut self, ns: &'a NamespaceDeclaration, session: &mut AnalysisSession) {
        let node: NodeRef = NodeRef::from(ns);
        self.notify_enter(&node, session);
        for member in &ns.declarations {
            self.visit_namespace_member(member, session);
        }
        self.notify_exit(&node, session);
    }

    fn visit_namespace_member(
        &mut self,
        member: &'a NamespaceBodyDeclaration,
        session: &mut AnalysisSession,
    ) {
        match member {
            NamespaceBodyDeclaration::Namespace(inner) => self.visit_namespace(inner, session),
            NamespaceBodyDeclaration::Class(class) => self.visit_class(class, session),
            NamespaceBodyDeclaration::Struct(_)
            | NamespaceBodyDeclaration::Interface(_)
            | NamespaceBodyDeclaration::Enum(_)
            | NamespaceBodyDeclaration::Delegate(_)
            | NamespaceBodyDeclaration::Record(_)
            | NamespaceBodyDeclaration::GlobalAttribute(_) => {}
        }
    }

    fn visit_class(&mut self, class: &'a ClassDeclaration, session: &mut AnalysisSession) {
        let node: NodeRef = NodeRef::from(class);
        self.notify_enter(&node, session);
        for member in &class.body_declarations {
            match member {
                ClassBodyDeclaration::Method(m) => self.visit_method(m, session),
                ClassBodyDeclaration::NestedClass(nested) => self.visit_class(nested, session),
                ClassBodyDeclaration::Constructor(c) => {
                    if let Some(body) = &c.body {
                        self.visit_statement(body, session);
                    }
                }
                ClassBodyDeclaration::Record(_) => {}
                ClassBodyDeclaration::Property(_)
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
        self.notify_exit(&node, session);
    }

    fn visit_method(&mut self, m: &'a MethodDeclaration, session: &mut AnalysisSession) {
        let node: NodeRef = NodeRef::from(m);
        self.notify_enter(&node, session);
        if let Some(body) = &m.body {
            self.visit_statement(body, session);
        }
        self.notify_exit(&node, session);
    }

    fn visit_statement(&mut self, stmt: &'a Statement, session: &mut AnalysisSession) {
        let node: NodeRef = NodeRef::from(stmt);
        self.notify_enter(&node, session);
        match stmt {
            If(s) => {
                self.visit_statement(&s.consequence, session);
                if let Some(alt) = &s.alternative {
                    self.visit_statement(alt, session);
                }
            }
            For(s) => {
                self.visit_statement(&s.body, session);
            }
            While(s) => {
                self.visit_statement(&s.body, session);
            }
            DoWhile(s) => {
                self.visit_statement(&s.body, session);
            }
            Switch(sw) => {
                for sec in &sw.sections {
                    for st in &sec.statements {
                        self.visit_statement(st, session);
                    }
                }
            }
            Try(t) => {
                self.visit_statement(&t.try_block, session);
                for c in &t.catches {
                    self.visit_statement(&c.block, session);
                }
                if let Some(fin) = &t.finally_clause {
                    self.visit_statement(&fin.block, session);
                }
            }
            Using(u) => {
                if let Some(body) = &u.body {
                    self.visit_statement(body, session);
                }
            }
            Statement::Block(stmts) => {
                for st in stmts {
                    self.visit_statement(st, session);
                }
            }
            _ => {}
        }
        self.notify_exit(&node, session);
    }
}
