use super::definitions::*;
use crate::subjects::math::formalism::abstraction_level::{AbstractionLevel, GetAbstractionLevel};

/// Validation and checking utilities for probability theory concepts

/// Check if a probability space is well-formed
pub fn validate_probability_space(space: &ProbabilitySpace) -> Result<(), String> {
    match space {
        ProbabilitySpace::Generic(space) => validate_generic_probability_space(space),
        ProbabilitySpace::Discrete(space) => validate_discrete_probability_space(space),
        ProbabilitySpace::Continuous(space) => validate_continuous_probability_space(space),
        ProbabilitySpace::Product(space) => validate_product_probability_space(space),
        ProbabilitySpace::Conditional(space) => validate_conditional_probability_space(space),
        ProbabilitySpace::StochasticProcess(space) => validate_stochastic_process(space),
        ProbabilitySpace::MarkovChain(space) => validate_markov_chain(space),
        ProbabilitySpace::Martingale(space) => validate_martingale(space),
        ProbabilitySpace::BrownianMotion(space) => validate_brownian_motion(space),
    }
}

/// Check if a generic probability space satisfies probability axioms
pub fn validate_generic_probability_space(space: &GenericProbabilitySpace) -> Result<(), String> {
    // Check that the sigma algebra is defined on the sample space
    if space.sigma_algebra.base_set != space.sample_space {
        return Err("Sigma algebra must be defined on the sample space".to_string());
    }

    // Check that the probability measure is defined on the sigma algebra
    if space.probability_measure.domain != space.sigma_algebra {
        return Err("Probability measure must be defined on the sigma algebra".to_string());
    }

    Ok(())
}

/// Check if a discrete probability space is well-formed
pub fn validate_discrete_probability_space(space: &DiscreteProbabilitySpace) -> Result<(), String> {
    validate_generic_probability_space(&space.core)?;

    // Check that probabilities sum to 1
    let total_probability: f64 = space.point_probabilities.values().sum();
    if (total_probability - 1.0).abs() > 1e-10 {
        return Err(format!(
            "Probabilities must sum to 1, got {}",
            total_probability
        ));
    }

    // Check that all probabilities are non-negative
    for (point, prob) in &space.point_probabilities {
        if *prob < 0.0 {
            return Err(format!(
                "Probability for point {} cannot be negative: {}",
                point, prob
            ));
        }
        if *prob > 1.0 {
            return Err(format!(
                "Probability for point {} cannot exceed 1: {}",
                point, prob
            ));
        }
    }

    // Check that every sample point has a probability
    for point in &space.sample_points {
        if !space.point_probabilities.contains_key(point) {
            return Err(format!(
                "Sample point {} has no associated probability",
                point
            ));
        }
    }

    Ok(())
}

/// Check if a continuous probability space is well-formed
pub fn validate_continuous_probability_space(
    space: &ContinuousProbabilitySpace,
) -> Result<(), String> {
    validate_generic_probability_space(&space.core)?;

    // Additional checks for continuous spaces could include:
    // - Topology compatibility with the sigma algebra
    // - Measure theoretic properties

    Ok(())
}

/// Check if a product probability space is well-formed
pub fn validate_product_probability_space(space: &ProductProbabilitySpace) -> Result<(), String> {
    validate_generic_probability_space(&space.core)?;

    if space.factors.is_empty() {
        return Err("Product space must have at least one factor".to_string());
    }

    // Validate each factor space
    for (i, factor) in space.factors.iter().enumerate() {
        validate_probability_space(factor)
            .map_err(|e| format!("Factor {} is invalid: {}", i, e))?;
    }

    Ok(())
}

/// Check if a conditional probability space is well-formed
pub fn validate_conditional_probability_space(
    space: &ConditionalProbabilitySpace,
) -> Result<(), String> {
    validate_generic_probability_space(&space.core)?;
    validate_probability_space(&space.original_space)?;
    validate_event(&space.conditioning_event)?;

    // The conditioning event must have positive probability
    // This would require access to the probability measure evaluation

    Ok(())
}

/// Check if a stochastic process is well-formed
pub fn validate_stochastic_process(process: &StochasticProcess) -> Result<(), String> {
    validate_generic_probability_space(&process.core)?;

    // Check that index set and state space are compatible with process type
    match process.process_type {
        StochasticProcessType::Markov => {
            // Markov processes should have appropriate index structure
        }
        StochasticProcessType::Martingale => {
            // Martingales need filtration structure (simplified here)
        }
        StochasticProcessType::BrownianMotion => {
            // Brownian motion typically has continuous time index
        }
        _ => {}
    }

    Ok(())
}

/// Check if a Markov chain is well-formed
pub fn validate_markov_chain(chain: &MarkovChain) -> Result<(), String> {
    validate_stochastic_process(&chain.core)?;
    validate_distribution(&chain.initial_distribution)?;

    // Validate transition matrix
    match &chain.transition_matrix {
        TransitionMatrix::Finite(matrix) => {
            if matrix.is_empty() {
                return Err("Transition matrix cannot be empty".to_string());
            }

            let n = matrix.len();
            for (i, row) in matrix.iter().enumerate() {
                if row.len() != n {
                    return Err(format!("Transition matrix row {} has wrong length", i));
                }

                // Check that rows sum to 1 (stochastic matrix)
                let row_sum: f64 = row.iter().sum();
                if (row_sum - 1.0).abs() > 1e-10 {
                    return Err(format!(
                        "Transition matrix row {} does not sum to 1: {}",
                        i, row_sum
                    ));
                }

                // Check that all entries are non-negative
                for (j, &entry) in row.iter().enumerate() {
                    if entry < 0.0 {
                        return Err(format!(
                            "Transition matrix entry ({},{}) is negative: {}",
                            i, j, entry
                        ));
                    }
                }
            }
        }
        TransitionMatrix::Kernel(_) => {
            // Validation for general kernels would be more complex
        }
    }

    Ok(())
}

/// Check if a martingale is well-formed
pub fn validate_martingale(martingale: &Martingale) -> Result<(), String> {
    validate_stochastic_process(&martingale.core)?;

    // Martingale property validation would require:
    // - Filtration structure
    // - Integrability conditions
    // - Martingale property: E[X_{n+1} | F_n] = X_n

    Ok(())
}

/// Check if Brownian motion is well-formed
pub fn validate_brownian_motion(brownian: &BrownianMotion) -> Result<(), String> {
    validate_stochastic_process(&brownian.core)?;

    // Check parameter validity
    if brownian.variance < 0.0 {
        return Err(format!(
            "Brownian motion variance cannot be negative: {}",
            brownian.variance
        ));
    }

    // Standard Brownian motion has specific parameter values
    if brownian.drift == 0.0 && brownian.variance == 1.0 {
        // This is standard Brownian motion
    }

    Ok(())
}

/// Check if an event is well-formed
pub fn validate_event(event: &Event) -> Result<(), String> {
    validate_probability_space(&event.probability_space)?;

    // Check that the event set is measurable with respect to the probability space's sigma algebra
    // This would require more sophisticated set membership checking

    Ok(())
}

/// Check if a random variable is well-formed
pub fn validate_random_variable(variable: &RandomVariable) -> Result<(), String> {
    validate_probability_space(&variable.probability_space)?;

    // Check type consistency
    match variable.variable_type {
        RandomVariableType::Discrete => {
            // Discrete variables should have countable range
        }
        RandomVariableType::Continuous => {
            // Continuous variables should have uncountable range
        }
        RandomVariableType::Mixed => {
            // Mixed variables have both discrete and continuous components
        }
        RandomVariableType::Singular => {
            // Singular variables are neither discrete nor continuous
        }
    }

    Ok(())
}

/// Check if a distribution is well-formed
pub fn validate_distribution(distribution: &Distribution) -> Result<(), String> {
    validate_random_variable(&distribution.random_variable)?;

    // Check parameter constraints
    for constraint in &distribution.parameters.constraints {
        match constraint {
            ParameterConstraint::Positive(param) => {
                if let Some(&value) = distribution.parameters.parameters.get(param) {
                    if value <= 0.0 {
                        return Err(format!(
                            "Parameter {} must be positive, got {}",
                            param, value
                        ));
                    }
                }
            }
            ParameterConstraint::Range {
                parameter,
                min,
                max,
            } => {
                if let Some(&value) = distribution.parameters.parameters.get(parameter) {
                    if value < *min || value > *max {
                        return Err(format!(
                            "Parameter {} must be in range [{}, {}], got {}",
                            parameter, min, max, value
                        ));
                    }
                }
            }
            ParameterConstraint::Integer(param) => {
                if let Some(&value) = distribution.parameters.parameters.get(param) {
                    if value.fract() != 0.0 {
                        return Err(format!(
                            "Parameter {} must be integer, got {}",
                            param, value
                        ));
                    }
                }
            }
            ParameterConstraint::Probability(param) => {
                if let Some(&value) = distribution.parameters.parameters.get(param) {
                    if value < 0.0 || value > 1.0 {
                        return Err(format!(
                            "Parameter {} must be a probability in [0, 1], got {}",
                            param, value
                        ));
                    }
                }
            }
        }
    }

    // Type-specific validation
    match &distribution.distribution_type {
        DistributionType::Discrete(variant) => {
            validate_discrete_distribution(variant, &distribution.parameters)?;
        }
        DistributionType::Continuous(variant) => {
            validate_continuous_distribution(variant, &distribution.parameters)?;
        }
        DistributionType::Mixed => {
            // Mixed distributions need both discrete and continuous components
        }
    }

    Ok(())
}

/// Validate discrete distribution parameters
pub fn validate_discrete_distribution(
    variant: &DiscreteDistributionVariant,
    params: &DistributionParameters,
) -> Result<(), String> {
    match variant {
        DiscreteDistributionVariant::Bernoulli => {
            if !params.parameters.contains_key("p") {
                return Err("Bernoulli distribution requires parameter 'p'".to_string());
            }
        }
        DiscreteDistributionVariant::Binomial => {
            if !params.parameters.contains_key("n") || !params.parameters.contains_key("p") {
                return Err("Binomial distribution requires parameters 'n' and 'p'".to_string());
            }
        }
        DiscreteDistributionVariant::Poisson => {
            if !params.parameters.contains_key("lambda") {
                return Err("Poisson distribution requires parameter 'lambda'".to_string());
            }
        }
        DiscreteDistributionVariant::Geometric => {
            if !params.parameters.contains_key("p") {
                return Err("Geometric distribution requires parameter 'p'".to_string());
            }
        }
        DiscreteDistributionVariant::UniformDiscrete => {
            if !params.parameters.contains_key("a") || !params.parameters.contains_key("b") {
                return Err(
                    "Uniform discrete distribution requires parameters 'a' and 'b'".to_string(),
                );
            }
        }
        DiscreteDistributionVariant::Hypergeometric => {
            if !params.parameters.contains_key("N")
                || !params.parameters.contains_key("K")
                || !params.parameters.contains_key("n")
            {
                return Err(
                    "Hypergeometric distribution requires parameters 'N', 'K', and 'n'".to_string(),
                );
            }
        }
    }
    Ok(())
}

/// Validate continuous distribution parameters
pub fn validate_continuous_distribution(
    variant: &ContinuousDistributionVariant,
    params: &DistributionParameters,
) -> Result<(), String> {
    match variant {
        ContinuousDistributionVariant::Normal => {
            if !params.parameters.contains_key("mu") || !params.parameters.contains_key("sigma") {
                return Err("Normal distribution requires parameters 'mu' and 'sigma'".to_string());
            }
        }
        ContinuousDistributionVariant::UniformContinuous => {
            if !params.parameters.contains_key("a") || !params.parameters.contains_key("b") {
                return Err(
                    "Uniform continuous distribution requires parameters 'a' and 'b'".to_string(),
                );
            }
        }
        ContinuousDistributionVariant::Exponential => {
            if !params.parameters.contains_key("lambda") {
                return Err("Exponential distribution requires parameter 'lambda'".to_string());
            }
        }
        ContinuousDistributionVariant::Gamma => {
            if !params.parameters.contains_key("alpha") || !params.parameters.contains_key("beta") {
                return Err("Gamma distribution requires parameters 'alpha' and 'beta'".to_string());
            }
        }
        ContinuousDistributionVariant::Beta => {
            if !params.parameters.contains_key("alpha") || !params.parameters.contains_key("beta") {
                return Err("Beta distribution requires parameters 'alpha' and 'beta'".to_string());
            }
        }
        ContinuousDistributionVariant::ChiSquared => {
            if !params.parameters.contains_key("nu") {
                return Err(
                    "Chi-squared distribution requires parameter 'nu' (degrees of freedom)"
                        .to_string(),
                );
            }
        }
        ContinuousDistributionVariant::StudentT => {
            if !params.parameters.contains_key("nu") {
                return Err(
                    "Student's t-distribution requires parameter 'nu' (degrees of freedom)"
                        .to_string(),
                );
            }
        }
    }
    Ok(())
}

/// Check probability relation validity
pub fn validate_probability_relation(relation: &ProbabilityRelation) -> Result<(), String> {
    match relation {
        ProbabilityRelation::EventsAreIndependent {
            events,
            probability_space,
        } => {
            // Check that all events belong to the same probability space
            for event in events {
                if let crate::subjects::math::formalism::extract::Parametrizable::Concrete(event) =
                    event
                {
                    if let crate::subjects::math::formalism::extract::Parametrizable::Concrete(
                        space,
                    ) = probability_space
                    {
                        if event.probability_space.as_ref() != space {
                            return Err(
                                "All events must belong to the same probability space".to_string()
                            );
                        }
                    }
                }
            }
        }
        ProbabilityRelation::RandomVariablesAreIndependent {
            variables,
            probability_space,
        } => {
            // Check that all random variables are defined on the same probability space
            for variable in variables {
                if let crate::subjects::math::formalism::extract::Parametrizable::Concrete(var) =
                    variable
                {
                    if let crate::subjects::math::formalism::extract::Parametrizable::Concrete(
                        space,
                    ) = probability_space
                    {
                        if var.probability_space.as_ref() != space {
                            return Err("All random variables must be defined on the same probability space".to_string());
                        }
                    }
                }
            }
        }
        ProbabilityRelation::EventHasProbability { probability, .. } => {
            if *probability < 0.0 || *probability > 1.0 {
                return Err(format!(
                    "Probability must be in [0, 1], got {}",
                    probability
                ));
            }
        }
        _ => {
            // Other relations can be validated as needed
        }
    }
    Ok(())
}

/// Check if abstraction levels are consistent
pub fn check_abstraction_consistency(space: &ProbabilitySpace) -> Result<(), String> {
    let level = space.level();

    match level {
        AbstractionLevel::Level1 => {
            // Abstract probability spaces should not have concrete parameters
        }
        AbstractionLevel::Level2 => {
            // Intermediate level spaces
        }
        AbstractionLevel::Level3 => {
            // Concrete probability spaces should have all parameters specified
            match space {
                ProbabilitySpace::Discrete(discrete_space) => {
                    if discrete_space.sample_points.is_empty() {
                        return Err("Concrete discrete space must have sample points".to_string());
                    }
                    if discrete_space.point_probabilities.is_empty() {
                        return Err(
                            "Concrete discrete space must have point probabilities".to_string()
                        );
                    }
                }
                _ => {}
            }
        }
        AbstractionLevel::Level4 => {
            // Meta-level or highly computational probability spaces
        }
    }

    Ok(())
}
