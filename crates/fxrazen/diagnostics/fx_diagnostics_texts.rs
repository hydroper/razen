use lazy_static::lazy_static;
use maplit::hashmap;
use crate::ns::*;

lazy_static! {
    pub static ref DATA: HashMap<i32, String> = hashmap! {
        // FxDiagnosticKind::K.id() => ".".into(),
        FxDiagnosticKind::EntityIsNotAType.id() => "Entity is not a type.".into(),
        FxDiagnosticKind::ImplicitCoercionToUnrelatedType.id() => "Implicit coercion of a value of type {1} to an unrelated type {2}.".into(),
        FxDiagnosticKind::EntityIsReadOnly.id() => "Entity is read-only.".into(),
        FxDiagnosticKind::EntityIsWriteOnly.id() => "Entity is write-only.".into(),
        FxDiagnosticKind::EntityMustNotBeDeleted.id() => "Entity must not be deleted.".into(),
        FxDiagnosticKind::UndefinedProperty.id() => "Access of possibly undefined property {1}.".into(),
        FxDiagnosticKind::AmbiguousReference.id() => "Ambiguous reference to {1}.".into(),
        FxDiagnosticKind::AccessOfVoid.id() => "Accessing property of void.".into(),
        FxDiagnosticKind::AccessOfNullable.id() => "Accessing property of nullable data type.".into(),
        FxDiagnosticKind::CouldNotExpandInlineConstant.id() => "Could not expand inline constant.".into(),
        FxDiagnosticKind::ReachedMaximumCycles.id() => "Reached maximum cycles.".into(),
        FxDiagnosticKind::NullNotExpectedHere.id() => "Null not expected here.".into(),
        FxDiagnosticKind::CouldNotParseNumber.id() => "Could not parse {1}.".into(),
        FxDiagnosticKind::NoMatchingEnumMember.id() => "Found no member {1} in {2}.".into(),
        FxDiagnosticKind::UnexpectedThis.id() => "Unexpected this.".into(),
        FxDiagnosticKind::ArrayLengthNotEqualsTupleLength.id() => "Array length is not equals length of tuple {1}.".into(),
        FxDiagnosticKind::UnexpectedElision.id() => "Unexpected elision.".into(),
        FxDiagnosticKind::UnexpectedArray.id() => "Unexpected array.".into(),
        FxDiagnosticKind::UnexpectedRest.id() => "Unexpected rest.".into(),
        FxDiagnosticKind::UnexpectedObject.id() => "Unexpected object.".into(),
        FxDiagnosticKind::DynamicOptionNotSupported.id() => "Dynamic option name is not supported.".into(),
        FxDiagnosticKind::UnknownOptionForClass.id() => "Unknown option {1} for {2}.".into(),
        FxDiagnosticKind::MustSpecifyOption.id() => "Must specify option {1}.".into(),
        FxDiagnosticKind::UnexpectedFieldName.id() => "Unexpected field name.".into(),
        FxDiagnosticKind::UnexpectedNewBase.id() => "Unexpected new base.".into(),
        FxDiagnosticKind::IncorrectNumArguments.id() => "Incorrect number of arguments. Expected {1}".into(),
        FxDiagnosticKind::IncorrectNumArgumentsNoMoreThan.id() => "Incorrect number of arguments. Expected no more than {1}".into(),
        FxDiagnosticKind::UndefinedPropertyWithStaticType.id() => "Access of possibly undefined property {1} through a reference with static type {2}.".into(),
        FxDiagnosticKind::InapplicableFilter.id() => "Attempt to filter through a reference with static type {1}.".into(),
        FxDiagnosticKind::InapplicableDescendants.id() => "Attempt to search descendants through a reference with static type {1}.".into(),
        FxDiagnosticKind::ASuperExpCanBeUsedOnlyIn.id() => "A super expression can be used only in class instance methods.".into(),
        FxDiagnosticKind::ASuperExpCanOnlyBeUsedInSubclasses.id() => "A super expression can be used only in subclasses of Object.".into(),
        FxDiagnosticKind::CallOnArrayType.id() => "A call on the Array type is equivalent to a new expression.".into(),
        FxDiagnosticKind::CallOnNonFunction.id() => "Call on non Function object.".into(),
        FxDiagnosticKind::NonParameterizedType.id() => "Applying types on non parameterized type.".into(),
        FxDiagnosticKind::AwaitOperandMustBeAPromise.id() => "Await operand must be a Promise.".into(),
        FxDiagnosticKind::OperandMustBeNumber.id() => "Operand must be a Number.".into(),
        FxDiagnosticKind::ReferenceIsAlreadyNonNullable.id() => "Reference is already non nullable.".into(),
        FxDiagnosticKind::YieldIsNotSupported.id() => "Yield operator is currently not supported.".into(),
        FxDiagnosticKind::UnrelatedMathOperation.id() => "Unrelated mathematical operation using type {1}.".into(),
        FxDiagnosticKind::ComparisonBetweenUnrelatedTypes.id() => "Comparison between a value of type {1} and an unrelated type {2}.".into(),
        FxDiagnosticKind::UnrelatedTernaryOperands.id() => "Unrelated ternary operands of types {1} and {2}.".into(),
        FxDiagnosticKind::SystemNamespaceNotFound.id() => "System namespace not found.".into(),
        FxDiagnosticKind::RestParameterMustBeArray.id() => "Rest parameter must be an Array.".into(),
        FxDiagnosticKind::AConflictExistsWithDefinition.id() => "A conflict exists with definition {1} in namespace {2}.".into(),
        FxDiagnosticKind::DuplicateVariableDefinition.id() => "Duplicate variable definition: {1}.".into(),
        FxDiagnosticKind::DuplicateClassDefinition.id() => "Duplicate variable definition: {1}.".into(),
        FxDiagnosticKind::DuplicateInterfaceDefinition.id() => "Duplicate interface definition: {1}.".into(),
        FxDiagnosticKind::DuplicateFunctionDefinition.id() => "Duplicate function definition: {1}.".into(),
        FxDiagnosticKind::UnexpectedFieldNameInDestructuring.id() => "Unexpected field name in destructuring.".into(),
        // FxDiagnosticKind::K.id() => ".".into(),
    };
}