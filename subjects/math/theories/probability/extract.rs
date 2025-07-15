use super::definitions::*;
use crate::subjects::math::formalism::extract::Parametrizable;
use crate::subjects::math::theories::zfc::definitions::Set;
use crate::turn_render::Identifier;
use serde_json::Number;

/// Extraction utilities for probability theory objects

/// Extract sample space from a probability space
pub fn extract_sample_space(space: &ProbabilitySpace) -> &Set {
    match space {
        ProbabilitySpace::Generic(space) => &space.sample_space,
        ProbabilitySpace::Discrete(space) => &space.core.sample_space,
        ProbabilitySpace::Continuous(space) => &space.core.sample_space,
        ProbabilitySpace::Product(space) => &space.core.sample_space,
        ProbabilitySpace::Conditional(space) => &space.core.sample_space,
        ProbabilitySpace::StochasticProcess(space) => &space.core.sample_space,
        ProbabilitySpace::MarkovChain(space) => &space.core.core.sample_space,
        ProbabilitySpace::Martingale(space) => &space.core.core.sample_space,
        ProbabilitySpace::BrownianMotion(space) => &space.core.core.sample_space,
    }
}

/// Extract sigma algebra from a probability space
pub fn extract_sigma_algebra(space: &ProbabilitySpace) -> &SigmaAlgebra {
    match space {
        ProbabilitySpace::Generic(space) => &space.sigma_algebra,
        ProbabilitySpace::Discrete(space) => &space.core.sigma_algebra,
        ProbabilitySpace::Continuous(space) => &space.core.sigma_algebra,
        ProbabilitySpace::Product(space) => &space.core.sigma_algebra,
        ProbabilitySpace::Conditional(space) => &space.core.sigma_algebra,
        ProbabilitySpace::StochasticProcess(space) => &space.core.sigma_algebra,
        ProbabilitySpace::MarkovChain(space) => &space.core.core.sigma_algebra,
        ProbabilitySpace::Martingale(space) => &space.core.core.sigma_algebra,
        ProbabilitySpace::BrownianMotion(space) => &space.core.core.sigma_algebra,
    }
}

/// Extract probability measure from a probability space
pub fn extract_probability_measure(space: &ProbabilitySpace) -> &ProbabilityMeasure {
    match space {
        ProbabilitySpace::Generic(space) => &space.probability_measure,
        ProbabilitySpace::Discrete(space) => &space.core.probability_measure,
        ProbabilitySpace::Continuous(space) => &space.core.probability_measure,
        ProbabilitySpace::Product(space) => &space.core.probability_measure,
        ProbabilitySpace::Conditional(space) => &space.core.probability_measure,
        ProbabilitySpace::StochasticProcess(space) => &space.core.probability_measure,
        ProbabilitySpace::MarkovChain(space) => &space.core.core.probability_measure,
        ProbabilitySpace::Martingale(space) => &space.core.core.probability_measure,
        ProbabilitySpace::BrownianMotion(space) => &space.core.core.probability_measure,
    }
}

/// Extract random variables from a probability relation
pub fn extract_random_variables_from_relation(
    relation: &ProbabilityRelation,
) -> Vec<&Parametrizable<RandomVariable>> {
    match relation {
        ProbabilityRelation::RandomVariablesAreIndependent { variables, .. } => {
            variables.iter().collect()
        }
        ProbabilityRelation::HasDistribution { variable, .. } => {
            vec![variable]
        }
        ProbabilityRelation::IdenticallyDistributed { variables } => variables.iter().collect(),
        ProbabilityRelation::ConvergesTo {
            sequence, limit, ..
        } => {
            let mut vars = sequence.iter().collect::<Vec<_>>();
            vars.push(limit);
            vars
        }
        ProbabilityRelation::ConditionallyIndependent { variables, .. } => {
            variables.iter().collect()
        }
        ProbabilityRelation::HasExpectedValue { variable, .. } => {
            vec![variable]
        }
        ProbabilityRelation::HasVariance { variable, .. } => {
            vec![variable]
        }
        ProbabilityRelation::SatisfiesLawOfLargeNumbers { sequence, .. } => {
            sequence.iter().collect()
        }
        ProbabilityRelation::SatisfiesCentralLimitTheorem { sequence } => sequence.iter().collect(),
        ProbabilityRelation::ConditionalExpectation {
            variable,
            conditional_expectation,
            ..
        } => {
            vec![variable, conditional_expectation]
        }
        _ => vec![],
    }
}

/// Extract events from a probability relation
pub fn extract_events_from_relation(relation: &ProbabilityRelation) -> Vec<&Parametrizable<Event>> {
    match relation {
        ProbabilityRelation::EventsAreIndependent { events, .. } => events.iter().collect(),
        ProbabilityRelation::ConditionallyIndependent {
            conditioning_event, ..
        } => {
            vec![conditioning_event]
        }
        ProbabilityRelation::EventHasProbability { event, .. } => {
            vec![event]
        }
        _ => vec![],
    }
}

/// Extract parameters from a distribution
pub fn extract_distribution_parameters(
    distribution: &Distribution,
) -> &std::collections::HashMap<Identifier, Number> {
    &distribution.parameters.parameters
}

/// Extract distribution type from a distribution
pub fn extract_distribution_type(distribution: &Distribution) -> &DistributionType {
    &distribution.distribution_type
}

/// Extract random variable from probability expression
pub fn extract_random_variable_from_expression(
    expr: &ProbabilityExpression,
) -> Option<&Parametrizable<RandomVariable>> {
    match expr {
        ProbabilityExpression::ExpectedValue { variable } => Some(variable),
        ProbabilityExpression::ConditionalExpectedValue { variable, .. } => Some(variable),
        ProbabilityExpression::Variance { variable } => Some(variable),
        ProbabilityExpression::Covariance { variable1, .. } => Some(variable1),
        ProbabilityExpression::Moment { variable, .. } => Some(variable),
        ProbabilityExpression::CharacteristicFunction { variable, .. } => Some(variable),
        ProbabilityExpression::MomentGeneratingFunction { variable, .. } => Some(variable),
        ProbabilityExpression::DistributionFunction { variable, .. } => Some(variable),
        ProbabilityExpression::ProbabilityDensityFunction { variable, .. } => Some(variable),
        ProbabilityExpression::ProbabilityMassFunction { variable, .. } => Some(variable),
        _ => None,
    }
}

/// Extract event from probability expression
pub fn extract_event_from_expression(
    expr: &ProbabilityExpression,
) -> Option<&Parametrizable<Event>> {
    match expr {
        ProbabilityExpression::EventProbability { event, .. } => Some(event),
        ProbabilityExpression::ConditionalProbability { event, .. } => Some(event),
        ProbabilityExpression::IndicatorVariable { event } => Some(event),
        _ => None,
    }
}

/// Extract all variables from a probability expression
pub fn extract_all_variables_from_expression(
    expr: &ProbabilityExpression,
) -> Vec<&Parametrizable<RandomVariable>> {
    match expr {
        ProbabilityExpression::Covariance {
            variable1,
            variable2,
        } => {
            vec![variable1, variable2]
        }
        ProbabilityExpression::RandomVariableSum { left, right } => {
            vec![left.as_ref(), right.as_ref()]
        }
        ProbabilityExpression::RandomVariableProduct { left, right } => {
            vec![left.as_ref(), right.as_ref()]
        }
        _ => extract_random_variable_from_expression(expr)
            .into_iter()
            .collect(),
    }
}

/// Extract factor spaces from a product probability space
pub fn extract_factor_spaces(space: &ProductProbabilitySpace) -> &Vec<Box<ProbabilitySpace>> {
    &space.factors
}

/// Extract conditioning information from conditional probability space
pub fn extract_conditioning_info(
    space: &ConditionalProbabilitySpace,
) -> (&ProbabilitySpace, &Event) {
    (space.original_space.as_ref(), &space.conditioning_event)
}

/// Extract process type from stochastic process
pub fn extract_process_type(process: &StochasticProcess) -> &StochasticProcessType {
    &process.process_type
}

/// Extract transition matrix from Markov chain
pub fn extract_transition_matrix(chain: &MarkovChain) -> &TransitionMatrix {
    &chain.transition_matrix
}

/// Extract drift and variance from Brownian motion
pub fn extract_brownian_parameters(brownian: &BrownianMotion) -> (Number, Number) {
    (brownian.drift.clone(), brownian.variance.clone())
}
