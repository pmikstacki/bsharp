#[macro_export]
macro_rules! diag {
    // Minimal form: diag!(session, CODE, at node)
    ($session:expr, $code:expr, at $node:expr) => {{
        let mut b = $crate::framework::diagnostic_builder::DiagnosticBuilder::new($code);
        // Try to resolve span via session.span_of(NodeRef)
        let nr: $crate::framework::NodeRef = $crate::framework::NodeRef::from(&$node);
        if let Some((start, len)) = $session.span_of(&nr) {
            b = b.at_span($session, start, len);
        }
        b.emit($session);
    }};
    // With custom message: diag!(session, CODE, at node, msg: expr)
    ($session:expr, $code:expr, at $node:expr, msg: $msg:expr) => {{
        let mut b = $crate::framework::diagnostic_builder::DiagnosticBuilder::new($code)
            .with_message($msg);
        let nr: $crate::framework::NodeRef = $crate::framework::NodeRef::from(&$node);
        if let Some((start, len)) = $session.span_of(&nr) {
            b = b.at_span($session, start, len);
        }
        b.emit($session);
    }};
}
