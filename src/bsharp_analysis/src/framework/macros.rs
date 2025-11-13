#[macro_export]
macro_rules! diag {
    // Minimal form: diag!(session, CODE, at node)
    ($session:expr, $code:expr, at $node:expr) => {{
        let mut b = $crate::framework::diagnostic_builder::DiagnosticBuilder::new($code);
        // Try to resolve span via session.span_of(NodeRef)
        let nr: $crate::framework::NodeRef = $crate::framework::NodeRef::from($node);
        if let Some((start, len)) = $session.span_of(&nr) {
            b = b.at_span($session, start, len);
        }
        b.emit($session);
    }};
    // With custom message: diag!(session, CODE, at node, msg: expr)
    ($session:expr, $code:expr, at $node:expr, msg: $msg:expr) => {{
        let mut b = $crate::framework::diagnostic_builder::DiagnosticBuilder::new($code)
            .with_message($msg);
        let nr: $crate::framework::NodeRef = $crate::framework::NodeRef::from($node);
        if let Some((start, len)) = $session.span_of(&nr) {
            b = b.at_span($session, start, len);
        }
        b.emit($session);
    }};
    // No-span, default message
    ($session:expr, $code:expr) => {{
        let b = $crate::framework::diagnostic_builder::DiagnosticBuilder::new($code);
        b.emit($session);
    }};
    // No-span with custom message
    ($session:expr, $code:expr, msg: $msg:expr) => {{
        let b = $crate::framework::diagnostic_builder::DiagnosticBuilder::new($code)
            .with_message($msg);
        b.emit($session);
    }};
    // Explicit span without message
    ($session:expr, $code:expr, at_span $start:expr, $len:expr) => {{
        let b = $crate::framework::diagnostic_builder::DiagnosticBuilder::new($code)
            .at_span($session, $start, $len);
        b.emit($session);
    }};
    // Explicit span with message
    ($session:expr, $code:expr, at_span $start:expr, $len:expr, msg: $msg:expr) => {{
        let b = $crate::framework::diagnostic_builder::DiagnosticBuilder::new($code)
            .with_message($msg)
            .at_span($session, $start, $len);
        b.emit($session);
    }};
}
