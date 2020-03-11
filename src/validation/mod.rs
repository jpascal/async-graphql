use crate::error::RuleErrors;
use crate::registry::Registry;
use crate::validation::context::ValidatorContext;
use crate::validation::visitor::{visit, VisitorNil};
use crate::Result;
use graphql_parser::query::Document;

mod context;
mod rules;
mod utils;
mod visitor;

pub fn check_rules(registry: &Registry, doc: &Document) -> Result<()> {
    let mut ctx = ValidatorContext::new(registry, doc);
    let mut visitor = VisitorNil
        .with(rules::ArgumentsOfCorrectType::default())
        .with(rules::DefaultValuesOfCorrectType)
        .with(rules::FieldsOnCorrectType)
        .with(rules::FragmentsOnCompositeTypes)
        .with(rules::KnownArgumentNames::default())
        .with(rules::NoFragmentCycles::default())
        .with(rules::KnownFragmentNames)
        .with(rules::KnownTypeNames)
        .with(rules::LoneAnonymousOperation::default())
        .with(rules::NoUndefinedVariables::default())
        .with(rules::NoUnusedFragments::default())
        .with(rules::NoUnusedVariables::default())
        .with(rules::UniqueArgumentNames::default())
        .with(rules::UniqueFragmentNames::default())
        .with(rules::UniqueOperationNames::default())
        .with(rules::UniqueVariableNames::default())
        .with(rules::VariablesAreInputTypes)
        .with(rules::VariableInAllowedPosition::default())
        .with(rules::ScalarLeafs)
        .with(rules::PossibleFragmentSpreads::default())
        .with(rules::ProvidedNonNullArguments)
        .with(rules::KnownDirectives::default());

    visit(&mut visitor, &mut ctx, doc);
    if !ctx.errors.is_empty() {
        Err(RuleErrors { errors: ctx.errors }.into())
    } else {
        Ok(())
    }
}
