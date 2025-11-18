| CS Code | Title | Meaning | B# Code | B# Name | Status |
| --- | --- | --- | --- | --- | --- |
| CS0101 | Namespace already contains a definition | Duplicate type in namespace | BSE03011 | Duplicate symbol | Implemented |
| CS0102 | Type already contains a definition for member | Duplicate member in type | BSE03011 | Duplicate symbol | Implemented |
| CS0103 | The name does not exist in the current context | Unresolved name | BSE03012 | Unresolved or ambiguous name | Implemented |
| CS0104 | Ambiguous reference between two symbols | Ambiguous reference | BSE03012 | Unresolved or ambiguous name | Implemented |
|   |   |   | BSE01007 | Invalid base constructor call | Implemented |
|   |   |   | BSE02002 | Non-abstract methods must have a body | Implemented |
|   |   |   | BSE04002 | Interface members cannot be private | Implemented |
|   |   |   | BSE04003 | Struct members cannot be protected | Implemented |
|   |   |   | BSE04005 | Static constructors cannot have access modifiers | Implemented |
|   |   |   | BSE04006 | Abstract members cannot be private | Implemented |
|   |   |   | BSE04007 | Virtual members cannot be private | Implemented |
|   |   |   | BSE04009 | Sealed modifier can only be used on overriding members | Implemented |
|   |   |   | BSE04010 | Abstract members cannot exist in non-abstract classes | Implemented |
|   |   |   | BSE01001 | Constructors cannot be declared async | Implemented |
|   |   |   | BSE01003 | Constructors cannot be virtual or abstract | Implemented |
|   |   |   | BSE01005 | Constructor name must match the containing class name | Implemented |
|   |   |   | BSE01009 | Constructors cannot override other constructors | Implemented |
|   |   |   | BSE02001 | Abstract methods cannot have a body | Implemented |
|   |   |   | BSE02003 | Virtual methods cannot be declared in sealed classes | Implemented |
|   |   |   | BSE02005 | Methods cannot be both virtual and static | Implemented |
|   |   |   | BSE02006 | Static methods cannot override other methods | Implemented |
|   |   |   | BSE02008 | Interface methods cannot have a body | Implemented |
|   |   |   | BSE02009 | Async methods must return Task or Task | Implemented |
|   |   |   | BSE02010 | Method parameter names must be unique | Implemented |
| CS0105 | The using directive for value appeared previously in this namespace | The using directive for '{0}' appeared previously in this namespace |   |   |   |
| CS0108 | value hides inherited member value | '{0}' hides inherited member '{1}'. Use the new keyword if hiding was intended. |   |   |   |
| CS0109 | The member value does not hide an accessible member | The member '{0}' does not hide an accessible member. The new keyword is not required. |   |   |   |
| CS0110 | The evaluation of the constant value for value involves a circular definition | The evaluation of the constant value for '{0}' involves a circular definition |   |   |   |
| CS0111 | Type value already defines a member called value with the same parameter types | Type '{1}' already defines a member called '{0}' with the same parameter types |   |   |   |
| CS0112 | A static member cannot be marked as value | A static member cannot be marked as '{0}' |   |   |   |
| CS0113 | A member value marked as override cannot be marked as new or virtual | A member '{0}' marked as override cannot be marked as new or virtual |   |   |   |
| CS0114 | value hides inherited member value | '{0}' hides inherited member '{1}'. To make the current member override that implementation, add the override keyword. Otherwise add the new keyword. |   |   |   |
| CS0115 | value: no suitable method found to override | '{0}': no suitable method found to override |   |   |   |
| CS0116 | A namespace cannot directly contain members such as fields, methods or statement | A namespace cannot directly contain members such as fields, methods or statements |   |   |   |
| CS0120 | An object reference is required for the non-static field, method, or property '{ | An object reference is required for the non-static field, method, or property '{0}' |   |   |   |
| CS0121 | The call is ambiguous between the following methods or properties: value and '{1 | The call is ambiguous between the following methods or properties: '{0}' and '{1}' |   |   |   |
| CS0122 | value is inaccessible due to its protection level | '{0}' is inaccessible due to its protection level |   |   |   |
| CS0123 | No overload for value matches delegate value | No overload for '{0}' matches delegate '{1}' |   |   |   |
| CS0132 | value: a static constructor must be parameterless | '{0}': a static constructor must be parameterless |   |   |   |
| CS0133 | The expression being assigned to value must be constant | The expression being assigned to '{0}' must be constant |   |   |   |
| CS0134 | value is of type value | '{0}' is of type '{1}'. A const field of a reference type other than string can only be initialized with null. |   |   |   |
| CS0138 | A 'using namespace' directive can only be applied to namespaces; value is a type | A 'using namespace' directive can only be applied to namespaces; '{0}' is a type not a namespace. Consider a 'using static' directive instead |   |   |   |
| CS0143 | The type value has no constructors defined | The type '{0}' has no constructors defined |   |   |   |
| CS0144 | Cannot create an instance of the abstract type or interface value | Cannot create an instance of the abstract type or interface '{0}' |   |   |   |
| CS0145 | A const field requires a value to be provided | A const field requires a value to be provided |   |   |   |
| CS0148 | The delegate value does not have a valid constructor | The delegate '{0}' does not have a valid constructor |   |   |   |
| CS0154 | The property or indexer value cannot be used in this context because it lacks th | The property or indexer '{0}' cannot be used in this context because it lacks the get accessor |   |   |   |
| CS0155 | The type caught or thrown must be derived from System | The type caught or thrown must be derived from System.Exception |   |   |   |
| CS0161 | value: not all code paths return a value | '{0}': not all code paths return a value |   |   |   |
| CS0173 | Type of conditional expression cannot be determined because there is no implicit | Type of conditional expression cannot be determined because there is no implicit conversion between '{0}' and '{1}' |   |   |   |
| CS0174 | A base class is required for a 'base' reference | A base class is required for a 'base' reference |   |   |   |
| CS0176 | Member value cannot be accessed with an instance reference; qualify it with a ty | Member '{0}' cannot be accessed with an instance reference; qualify it with a type name instead |   |   |   |
| CS0180 | value cannot be both extern and abstract | '{0}' cannot be both extern and abstract |   |   |   |
| CS0181 | Attribute constructor parameter value has type value, which is not a valid attri | Attribute constructor parameter '{0}' has type '{1}', which is not a valid attribute parameter type |   |   |   |
| CS0190 | The \_\_arglist construct is valid only within a variable argument method | The \_\_arglist construct is valid only within a variable argument method |   |   |   |
| CS0191 | A readonly field cannot be assigned to (except in a constructor or init-only set | A readonly field cannot be assigned to (except in a constructor or init-only setter of the type in which the field is defined or a variable initializer) |   |   |   |
| CS0192 | A readonly field cannot be used as a ref or out value (except in a constructor) | A readonly field cannot be used as a ref or out value (except in a constructor) |   |   |   |
| CS0198 | A static readonly field cannot be assigned to (except in a static constructor or | A static readonly field cannot be assigned to (except in a static constructor or a variable initializer) |   |   |   |
| CS0199 | A static readonly field cannot be used as a ref or out value (except in a static | A static readonly field cannot be used as a ref or out value (except in a static constructor) |   |   |   |