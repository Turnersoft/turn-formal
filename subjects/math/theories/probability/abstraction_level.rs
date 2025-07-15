use super::definitions::*;
use crate::subjects::math::formalism::abstraction_level::{AbstractionLevel, GetAbstractionLevel};

/// Implement abstraction level for probability spaces
impl GetAbstractionLevel for ProbabilitySpace {
    fn level(&self) -> AbstractionLevel {
        match self {
            ProbabilitySpace::Generic(space) => space.level(),
            ProbabilitySpace::Discrete(space) => space.level(),
            ProbabilitySpace::Continuous(space) => space.level(),
            ProbabilitySpace::Product(space) => space.level(),
            ProbabilitySpace::Conditional(space) => space.level(),
            ProbabilitySpace::StochasticProcess(space) => space.level(),
            ProbabilitySpace::MarkovChain(space) => space.level(),
            ProbabilitySpace::Martingale(space) => space.level(),
            ProbabilitySpace::BrownianMotion(space) => space.level(),
        }
    }
}

/// Generic probability space abstraction level
impl GetAbstractionLevel for GenericProbabilitySpace {
    fn level(&self) -> AbstractionLevel {
        // Check if parameters are concrete (Level 3) or abstract (Level 1)
        let sample_space_level = self.sample_space.level();
        let sigma_algebra_level = self.sigma_algebra.level();
        let measure_level = self.probability_measure.level();

        // Return the highest (most concrete) level
        if sample_space_level == AbstractionLevel::Level3
            || sigma_algebra_level == AbstractionLevel::Level3
            || measure_level == AbstractionLevel::Level3
        {
            AbstractionLevel::Level3
        } else if sample_space_level == AbstractionLevel::Level2
            || sigma_algebra_level == AbstractionLevel::Level2
            || measure_level == AbstractionLevel::Level2
        {
            AbstractionLevel::Level2
        } else {
            AbstractionLevel::Level1
        }
    }
}

/// Sigma algebra abstraction level
impl GetAbstractionLevel for SigmaAlgebra {
    fn level(&self) -> AbstractionLevel {
        match &self.algebra_type {
            SigmaAlgebraType::PowerSet => {
                // Power set is abstract when base set is abstract
                self.base_set.level()
            }
            SigmaAlgebraType::Borel => {
                // Borel algebra is Level 1 (abstract schema)
                AbstractionLevel::Level1
            }
            SigmaAlgebraType::Generated { generators } => {
                if generators.is_empty() {
                    AbstractionLevel::Level1
                } else {
                    // Concrete generators make it Level 3
                    AbstractionLevel::Level3
                }
            }
            SigmaAlgebraType::Product { factors } => {
                if factors.is_empty() {
                    AbstractionLevel::Level1
                } else {
                    // Check abstraction level of factors
                    let max_level = factors
                        .iter()
                        .map(|f| f.level())
                        .max()
                        .unwrap_or(AbstractionLevel::Level1);
                    max_level
                }
            }
            SigmaAlgebraType::Trace { parent, subset } => {
                let parent_level = parent.level();
                let subset_level = subset.level();
                if parent_level == AbstractionLevel::Level3
                    || subset_level == AbstractionLevel::Level3
                {
                    AbstractionLevel::Level3
                } else if parent_level == AbstractionLevel::Level2
                    || subset_level == AbstractionLevel::Level2
                {
                    AbstractionLevel::Level2
                } else {
                    AbstractionLevel::Level1
                }
            }
        }
    }
}

/// Probability measure abstraction level
impl GetAbstractionLevel for ProbabilityMeasure {
    fn level(&self) -> AbstractionLevel {
        // Most probability measures are abstract unless they have specific concrete parameters
        match self.measure_type {
            ProbabilityMeasureVariant::StandardMeasure => AbstractionLevel::Level1,
            ProbabilityMeasureVariant::CountingMeasure => AbstractionLevel::Level1,
            ProbabilityMeasureVariant::UniformMeasure => AbstractionLevel::Level1,
            ProbabilityMeasureVariant::LebesgueMeasure => AbstractionLevel::Level1,
            ProbabilityMeasureVariant::EmpiricalMeasure => AbstractionLevel::Level3, // Concrete data
            ProbabilityMeasureVariant::ProductMeasure => self.domain.level(),
            ProbabilityMeasureVariant::ConditionalMeasure => AbstractionLevel::Level2, // Intermediate
        }
    }
}

/// Discrete probability space abstraction level
impl GetAbstractionLevel for DiscreteProbabilitySpace {
    fn level(&self) -> AbstractionLevel {
        // If sample points are specified, it's concrete
        if !self.sample_points.is_empty() && !self.point_probabilities.is_empty() {
            AbstractionLevel::Level3
        } else {
            self.core.level()
        }
    }
}

/// Continuous probability space abstraction level
impl GetAbstractionLevel for ContinuousProbabilitySpace {
    fn level(&self) -> AbstractionLevel {
        // Continuous spaces are typically abstract unless topology is very specific
        let core_level = self.core.level();
        let topology_level = self.topology.level();

        if core_level == AbstractionLevel::Level3 || topology_level == AbstractionLevel::Level3 {
            AbstractionLevel::Level3
        } else if core_level == AbstractionLevel::Level2
            || topology_level == AbstractionLevel::Level2
        {
            AbstractionLevel::Level2
        } else {
            AbstractionLevel::Level1
        }
    }
}

/// Product probability space abstraction level
impl GetAbstractionLevel for ProductProbabilitySpace {
    fn level(&self) -> AbstractionLevel {
        if self.factors.is_empty() {
            AbstractionLevel::Level1
        } else {
            // Return the highest abstraction level among factors
            let max_level = self
                .factors
                .iter()
                .map(|f| f.level())
                .max()
                .unwrap_or(AbstractionLevel::Level1);
            max_level
        }
    }
}

/// Conditional probability space abstraction level
impl GetAbstractionLevel for ConditionalProbabilitySpace {
    fn level(&self) -> AbstractionLevel {
        let original_level = self.original_space.level();
        let event_level = self.conditioning_event.level();

        if original_level == AbstractionLevel::Level3 || event_level == AbstractionLevel::Level3 {
            AbstractionLevel::Level3
        } else if original_level == AbstractionLevel::Level2
            || event_level == AbstractionLevel::Level2
        {
            AbstractionLevel::Level2
        } else {
            AbstractionLevel::Level1
        }
    }
}

/// Event abstraction level
impl GetAbstractionLevel for Event {
    fn level(&self) -> AbstractionLevel {
        let set_level = self.event_set.level();
        let space_level = self.probability_space.level();

        if set_level == AbstractionLevel::Level3 || space_level == AbstractionLevel::Level3 {
            AbstractionLevel::Level3
        } else if set_level == AbstractionLevel::Level2 || space_level == AbstractionLevel::Level2 {
            AbstractionLevel::Level2
        } else {
            AbstractionLevel::Level1
        }
    }
}

/// Random variable abstraction level
impl GetAbstractionLevel for RandomVariable {
    fn level(&self) -> AbstractionLevel {
        let space_level = self.probability_space.level();
        let target_level = self.target_space.level();

        // Random variables are abstract unless their domain/codomain are concrete
        match self.variable_type {
            RandomVariableType::Discrete => {
                if space_level == AbstractionLevel::Level3
                    || target_level == AbstractionLevel::Level3
                {
                    AbstractionLevel::Level3
                } else {
                    AbstractionLevel::Level1
                }
            }
            RandomVariableType::Continuous => {
                if space_level == AbstractionLevel::Level3
                    || target_level == AbstractionLevel::Level3
                {
                    AbstractionLevel::Level3
                } else {
                    AbstractionLevel::Level1
                }
            }
            RandomVariableType::Mixed => AbstractionLevel::Level2, // Intermediate complexity
            RandomVariableType::Singular => AbstractionLevel::Level2, // Intermediate complexity
        }
    }
}

/// Distribution abstraction level
impl GetAbstractionLevel for Distribution {
    fn level(&self) -> AbstractionLevel {
        let variable_level = self.random_variable.level();

        // Check if distribution has concrete parameters
        let has_concrete_parameters = !self.parameters.parameters.is_empty();

        match &self.distribution_type {
            DistributionType::Discrete(_) => {
                if has_concrete_parameters {
                    AbstractionLevel::Level3
                } else if variable_level == AbstractionLevel::Level3 {
                    AbstractionLevel::Level3
                } else {
                    AbstractionLevel::Level1
                }
            }
            DistributionType::Continuous(_) => {
                if has_concrete_parameters {
                    AbstractionLevel::Level3
                } else if variable_level == AbstractionLevel::Level3 {
                    AbstractionLevel::Level3
                } else {
                    AbstractionLevel::Level1
                }
            }
            DistributionType::Mixed => {
                if has_concrete_parameters || variable_level == AbstractionLevel::Level3 {
                    AbstractionLevel::Level3
                } else {
                    AbstractionLevel::Level2
                }
            }
        }
    }
}

/// Stochastic process abstraction level
impl GetAbstractionLevel for StochasticProcess {
    fn level(&self) -> AbstractionLevel {
        let core_level = self.core.level();
        let index_level = self.index_set.level();
        let state_level = self.state_space.level();

        // Process type affects abstraction level
        let process_level = match self.process_type {
            StochasticProcessType::General => AbstractionLevel::Level1,
            StochasticProcessType::Markov => AbstractionLevel::Level1,
            StochasticProcessType::Martingale => AbstractionLevel::Level1,
            StochasticProcessType::BrownianMotion => AbstractionLevel::Level1,
            StochasticProcessType::PoissonProcess => AbstractionLevel::Level1,
            StochasticProcessType::RandomWalk => AbstractionLevel::Level1,
        };

        // Return highest level among all components
        let levels = vec![core_level, index_level, state_level, process_level];
        levels.into_iter().max().unwrap_or(AbstractionLevel::Level1)
    }
}

/// Markov chain abstraction level
impl GetAbstractionLevel for MarkovChain {
    fn level(&self) -> AbstractionLevel {
        let core_level = self.core.level();
        let state_level = self.state_space.level();
        let distribution_level = self.initial_distribution.level();

        // Transition matrix determines concreteness
        let transition_level = match &self.transition_matrix {
            TransitionMatrix::Finite(matrix) => {
                if matrix.is_empty() || matrix.iter().all(|row| row.is_empty()) {
                    AbstractionLevel::Level1
                } else {
                    AbstractionLevel::Level3 // Concrete matrix
                }
            }
            TransitionMatrix::Kernel(_) => AbstractionLevel::Level2, // Intermediate
        };

        // Return highest level
        let levels = vec![
            core_level,
            state_level,
            distribution_level,
            transition_level,
        ];
        levels.into_iter().max().unwrap_or(AbstractionLevel::Level1)
    }
}

/// Martingale abstraction level
impl GetAbstractionLevel for Martingale {
    fn level(&self) -> AbstractionLevel {
        let core_level = self.core.level();

        // Martingales are typically abstract unless the filtration is very specific
        if core_level == AbstractionLevel::Level3 {
            AbstractionLevel::Level3
        } else {
            AbstractionLevel::Level1
        }
    }
}

/// Brownian motion abstraction level
impl GetAbstractionLevel for BrownianMotion {
    fn level(&self) -> AbstractionLevel {
        todo!()
    }
}

/// Helper function to determine abstraction level for probability relations
pub fn level_for_probability_relation(relation: &ProbabilityRelation) -> AbstractionLevel {
    match relation {
        ProbabilityRelation::EventsAreIndependent {
            events,
            probability_space,
        } => {
            if events.is_empty() {
                AbstractionLevel::Level1
            } else {
                let event_levels: Vec<AbstractionLevel> = events
                    .iter()
                    .filter_map(|e| match e {
                        crate::subjects::math::formalism::extract::Parametrizable::Concrete(
                            event,
                        ) => Some(event.level()),
                        _ => None,
                    })
                    .collect();

                if event_levels.iter().any(|l| *l == AbstractionLevel::Level3) {
                    AbstractionLevel::Level3
                } else if event_levels.iter().any(|l| *l == AbstractionLevel::Level2) {
                    AbstractionLevel::Level2
                } else {
                    AbstractionLevel::Level1
                }
            }
        }

        ProbabilityRelation::RandomVariablesAreIndependent {
            variables,
            probability_space,
        } => {
            if variables.is_empty() {
                AbstractionLevel::Level1
            } else {
                let var_levels: Vec<AbstractionLevel> = variables
                    .iter()
                    .filter_map(|v| match v {
                        crate::subjects::math::formalism::extract::Parametrizable::Concrete(
                            var,
                        ) => Some(var.level()),
                        _ => None,
                    })
                    .collect();

                if var_levels.iter().any(|l| *l == AbstractionLevel::Level3) {
                    AbstractionLevel::Level3
                } else if var_levels.iter().any(|l| *l == AbstractionLevel::Level2) {
                    AbstractionLevel::Level2
                } else {
                    AbstractionLevel::Level1
                }
            }
        }

        ProbabilityRelation::HasDistribution {
            variable,
            distribution,
        } => {
            let var_level = match variable {
                crate::subjects::math::formalism::extract::Parametrizable::Concrete(v) => v.level(),
                _ => AbstractionLevel::Level1,
            };
            let dist_level = match distribution {
                crate::subjects::math::formalism::extract::Parametrizable::Concrete(d) => d.level(),
                _ => AbstractionLevel::Level1,
            };

            if var_level == AbstractionLevel::Level3 || dist_level == AbstractionLevel::Level3 {
                AbstractionLevel::Level3
            } else if var_level == AbstractionLevel::Level2
                || dist_level == AbstractionLevel::Level2
            {
                AbstractionLevel::Level2
            } else {
                AbstractionLevel::Level1
            }
        }

        ProbabilityRelation::EventHasProbability {
            event,
            probability,
            probability_space,
        } => {
            // Specific probability value makes it concrete
            AbstractionLevel::Level3
        }

        ProbabilityRelation::HasExpectedValue {
            variable,
            expected_value,
        } => {
            // Specific expected value makes it concrete
            AbstractionLevel::Level3
        }

        ProbabilityRelation::HasVariance { variable, variance } => {
            // Specific variance makes it concrete
            AbstractionLevel::Level3
        }

        // Most other relations are Level 1 (abstract) unless they involve concrete objects
        _ => AbstractionLevel::Level1,
    }
}

/// Helper function to determine abstraction level for probability expressions
pub fn level_for_probability_expression(expression: &ProbabilityExpression) -> AbstractionLevel {
    match expression {
        ProbabilityExpression::EventProbability {
            event,
            probability_space,
        } => {
            let event_level = match event {
                crate::subjects::math::formalism::extract::Parametrizable::Concrete(e) => e.level(),
                _ => AbstractionLevel::Level1,
            };
            let space_level = match probability_space {
                crate::subjects::math::formalism::extract::Parametrizable::Concrete(s) => s.level(),
                _ => AbstractionLevel::Level1,
            };

            if event_level == AbstractionLevel::Level3 || space_level == AbstractionLevel::Level3 {
                AbstractionLevel::Level3
            } else if event_level == AbstractionLevel::Level2
                || space_level == AbstractionLevel::Level2
            {
                AbstractionLevel::Level2
            } else {
                AbstractionLevel::Level1
            }
        }

        ProbabilityExpression::ExpectedValue { variable } => match variable {
            crate::subjects::math::formalism::extract::Parametrizable::Concrete(v) => v.level(),
            _ => AbstractionLevel::Level1,
        },

        ProbabilityExpression::Variance { variable } => match variable {
            crate::subjects::math::formalism::extract::Parametrizable::Concrete(v) => v.level(),
            _ => AbstractionLevel::Level1,
        },

        ProbabilityExpression::CharacteristicFunction {
            variable,
            parameter: _,
        } => {
            // Specific parameter value makes it Level 3
            AbstractionLevel::Level3
        }

        ProbabilityExpression::MomentGeneratingFunction {
            variable,
            parameter: _,
        } => {
            // Specific parameter value makes it Level 3
            AbstractionLevel::Level3
        }

        ProbabilityExpression::DistributionFunction { variable, value: _ } => {
            // Specific value makes it Level 3
            AbstractionLevel::Level3
        }

        ProbabilityExpression::ProbabilityDensityFunction { variable, value: _ } => {
            // Specific value makes it Level 3
            AbstractionLevel::Level3
        }

        ProbabilityExpression::ProbabilityMassFunction { variable, value: _ } => {
            // Specific value makes it Level 3
            AbstractionLevel::Level3
        }

        // Operations between random variables
        ProbabilityExpression::RandomVariableSum { left, right } => {
            let left_level = match left.as_ref() {
                crate::subjects::math::formalism::extract::Parametrizable::Concrete(v) => v.level(),
                _ => AbstractionLevel::Level1,
            };
            let right_level = match right.as_ref() {
                crate::subjects::math::formalism::extract::Parametrizable::Concrete(v) => v.level(),
                _ => AbstractionLevel::Level1,
            };

            if left_level == AbstractionLevel::Level3 || right_level == AbstractionLevel::Level3 {
                AbstractionLevel::Level3
            } else if left_level == AbstractionLevel::Level2
                || right_level == AbstractionLevel::Level2
            {
                AbstractionLevel::Level2
            } else {
                AbstractionLevel::Level1
            }
        }

        ProbabilityExpression::RandomVariableProduct { left, right } => {
            let left_level = match left.as_ref() {
                crate::subjects::math::formalism::extract::Parametrizable::Concrete(v) => v.level(),
                _ => AbstractionLevel::Level1,
            };
            let right_level = match right.as_ref() {
                crate::subjects::math::formalism::extract::Parametrizable::Concrete(v) => v.level(),
                _ => AbstractionLevel::Level1,
            };

            if left_level == AbstractionLevel::Level3 || right_level == AbstractionLevel::Level3 {
                AbstractionLevel::Level3
            } else if left_level == AbstractionLevel::Level2
                || right_level == AbstractionLevel::Level2
            {
                AbstractionLevel::Level2
            } else {
                AbstractionLevel::Level1
            }
        }

        // Most other expressions default to Level 1 unless involving concrete objects
        _ => AbstractionLevel::Level1,
    }
}
