use crate::framework::{AnalysisSession, Rule, RuleSet};
use crate::framework::NodeRef;
use crate::syntax::ast::TopLevelDeclaration;
use crate::{diag, DiagnosticCode, rule, ruleset};
use bsharp_syntax::declarations::{ClassBodyDeclaration, Modifier};
use bsharp_syntax::types::{PrimitiveType, Type};

#[path = "semantic/utils.rs"]
pub mod utils;
#[path = "semantic/ctor_no_async.rs"]
mod ctor_no_async;
#[path = "semantic/ctor_name_matches_class.rs"]
mod ctor_name_matches_class;
#[path = "semantic/ctor_no_virtual_or_abstract.rs"]
mod ctor_no_virtual_or_abstract;
#[path = "semantic/method_no_abstract_body.rs"]
mod method_no_abstract_body;
#[path = "semantic/method_no_static_override.rs"]
mod method_no_static_override;
#[path = "semantic/async_returns_task.rs"]
mod async_returns_task;
#[path = "semantic/method_must_have_body_unless_abstract.rs"]
mod method_must_have_body_unless_abstract;
#[path = "semantic/ctor_invalid_base_call.rs"]
mod ctor_invalid_base_call;

use self::ctor_no_async::CtorNoAsync;
use self::ctor_name_matches_class::CtorNameMatchesClass;
use self::ctor_no_virtual_or_abstract::CtorNoVirtualOrAbstract;
use self::method_no_abstract_body::MethodNoAbstractBody;
use self::method_no_static_override::MethodNoStaticOverride;
use self::async_returns_task::AsyncReturnsTask;
use self::method_must_have_body_unless_abstract::MethodMustHaveBodyUnlessAbstract;
use self::ctor_invalid_base_call::CtorInvalidBaseCall;

ruleset! {
    semantic: CtorNoAsync, CtorNameMatchesClass, CtorNoVirtualOrAbstract, MethodNoAbstractBody, MethodNoStaticOverride, AsyncReturnsTask, MethodMustHaveBodyUnlessAbstract, CtorInvalidBaseCall
}
