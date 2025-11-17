use crate::ruleset;
use crate::rules::RuleSet;

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
#[path = "semantic/interface_members_no_private.rs"]
mod interface_members_no_private;
#[path = "semantic/struct_members_no_protected.rs"]
mod struct_members_no_protected;
#[path = "semantic/static_ctor_no_access_modifiers.rs"]
mod static_ctor_no_access_modifiers;
#[path = "semantic/abstract_members_no_private.rs"]
mod abstract_members_no_private;
#[path = "semantic/virtual_members_no_private.rs"]
mod virtual_members_no_private;
#[path = "semantic/sealed_only_on_overrides.rs"]
mod sealed_only_on_overrides;
#[path = "semantic/abstract_members_only_in_abstract_class.rs"]
mod abstract_members_only_in_abstract_class;
#[path = "semantic/sealed_class_no_virtual_methods.rs"]
mod sealed_class_no_virtual_methods;
#[path = "semantic/method_no_static_virtual.rs"]
mod method_no_static_virtual;
#[path = "semantic/interface_methods_no_body.rs"]
mod interface_methods_no_body;
#[path = "semantic/method_param_names_unique.rs"]
mod method_param_names_unique;
#[path = "semantic/ctors_no_override.rs"]
mod ctors_no_override;

use self::ctor_no_async::CtorNoAsync;
use self::ctor_name_matches_class::CtorNameMatchesClass;
use self::ctor_no_virtual_or_abstract::CtorNoVirtualOrAbstract;
use self::method_no_abstract_body::MethodNoAbstractBody;
use self::method_no_static_override::MethodNoStaticOverride;
use self::async_returns_task::AsyncReturnsTask;
use self::method_must_have_body_unless_abstract::MethodMustHaveBodyUnlessAbstract;
use self::ctor_invalid_base_call::CtorInvalidBaseCall;
use self::interface_members_no_private::InterfaceMembersNoPrivate;
use self::struct_members_no_protected::StructMembersNoProtected;
use self::static_ctor_no_access_modifiers::StaticCtorNoAccessModifiers;
use self::abstract_members_no_private::AbstractMembersNoPrivate;
use self::virtual_members_no_private::VirtualMembersNoPrivate;
use self::sealed_only_on_overrides::SealedOnlyOnOverrides;
use self::abstract_members_only_in_abstract_class::AbstractMembersOnlyInAbstractClass;
use self::sealed_class_no_virtual_methods::SealedClassNoVirtualMethods;
use self::method_no_static_virtual::MethodNoStaticVirtual;
use self::interface_methods_no_body::InterfaceMethodsNoBody;
use self::method_param_names_unique::MethodParamNamesUnique;
use self::ctors_no_override::CtorsNoOverride;

ruleset! {
    semantic: 
        CtorNoAsync, 
        CtorNameMatchesClass, 
        CtorNoVirtualOrAbstract, 
        MethodNoAbstractBody, 
        MethodNoStaticOverride, 
        AsyncReturnsTask, 
        MethodMustHaveBodyUnlessAbstract, 
        CtorInvalidBaseCall, 
        InterfaceMembersNoPrivate, 
        StructMembersNoProtected, 
        StaticCtorNoAccessModifiers, 
        AbstractMembersNoPrivate, 
        VirtualMembersNoPrivate, 
        SealedOnlyOnOverrides, 
        AbstractMembersOnlyInAbstractClass,
        SealedClassNoVirtualMethods,
        MethodNoStaticVirtual,
        InterfaceMethodsNoBody,
        MethodParamNamesUnique,
        CtorsNoOverride
}
