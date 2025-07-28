use super::definitions::*;
use crate::subjects::math::formalism::traits::complexity::Complexity;

impl Complexity for ProbabilitySpace {
    fn complexity(&self) -> usize {
        match self {
            ProbabilitySpace::Generic(space) => 1 + space.complexity(),
            ProbabilitySpace::Discrete(space) => 2 + space.complexity(),
            ProbabilitySpace::Continuous(space) => 3 + space.complexity(),
            ProbabilitySpace::Product(space) => 2 + space.complexity(),
            ProbabilitySpace::Conditional(space) => 3 + space.complexity(),
            ProbabilitySpace::StochasticProcess(space) => 4 + space.complexity(),
            ProbabilitySpace::MarkovChain(space) => 4 + space.complexity(),
            ProbabilitySpace::Martingale(space) => 5 + space.complexity(),
            ProbabilitySpace::BrownianMotion(space) => 4 + space.complexity(),
        }
    }
}

impl Complexity for GenericProbabilitySpace {
    fn complexity(&self) -> usize {
        1 + self.sample_space.complexity()
            + self.sigma_algebra.complexity()
            + self.probability_measure.complexity()
    }
}

impl Complexity for SigmaAlgebra {
    fn complexity(&self) -> usize {
        match &self.algebra_type {
            SigmaAlgebraType::PowerSet => 1,
            SigmaAlgebraType::Borel => 2,
            SigmaAlgebraType::Generated { generators } => 2 + generators.len(),
            SigmaAlgebraType::Product { factors } => {
                2 + factors.iter().map(|f| f.complexity()).sum::<usize>()
            }
            SigmaAlgebraType::Trace { parent, subset } => {
                3 + parent.complexity() + subset.complexity()
            }
        }
    }
}

impl Complexity for ProbabilityMeasure {
    fn complexity(&self) -> usize {
        match self.measure_type {
            ProbabilityMeasureVariant::StandardMeasure => 1,
            ProbabilityMeasureVariant::CountingMeasure => 1,
            ProbabilityMeasureVariant::UniformMeasure => 1,
            ProbabilityMeasureVariant::LebesgueMeasure => 2,
            ProbabilityMeasureVariant::EmpiricalMeasure => 3, // More complex
            ProbabilityMeasureVariant::ProductMeasure => 2,
            ProbabilityMeasureVariant::ConditionalMeasure => 3,
        }
    }
}

impl Complexity for Event {
    fn complexity(&self) -> usize {
        1 + self.event_set.complexity() + self.probability_space.complexity()
    }
}

impl Complexity for RandomVariable {
    fn complexity(&self) -> usize {
        let type_complexity = match self.variable_type {
            RandomVariableType::Discrete => 1,
            RandomVariableType::Continuous => 2,
            RandomVariableType::Mixed => 3,
            RandomVariableType::Singular => 3,
        };

        1 + type_complexity + self.probability_space.complexity() + self.target_space.complexity()
    }
}

impl Complexity for Distribution {
    fn complexity(&self) -> usize {
        let type_complexity = match &self.distribution_type {
            DistributionType::Discrete(_) => 1,
            DistributionType::Continuous(_) => 2,
            DistributionType::Mixed => 3,
        };

        let param_complexity = self.parameters.parameters.len();

        1 + type_complexity + param_complexity + self.random_variable.complexity()
    }
}

impl Complexity for DiscreteProbabilitySpace {
    fn complexity(&self) -> usize {
        1 + self.core.complexity() + self.sample_points.len() + self.point_probabilities.len()
    }
}

impl Complexity for ContinuousProbabilitySpace {
    fn complexity(&self) -> usize {
        // Using a fixed complexity for topology since TopologicalSpace doesn't implement Complexity yet
        1 + self.core.complexity() + 2 // Assuming topology adds complexity of 2
    }
}

impl Complexity for ProductProbabilitySpace {
    fn complexity(&self) -> usize {
        1 + self.core.complexity() + self.factors.iter().map(|f| f.complexity()).sum::<usize>()
    }
}

impl Complexity for ConditionalProbabilitySpace {
    fn complexity(&self) -> usize {
        1 + self.core.complexity()
            + self.original_space.complexity()
            + self.conditioning_event.complexity()
    }
}

impl Complexity for StochasticProcess {
    fn complexity(&self) -> usize {
        let process_complexity = match self.process_type {
            StochasticProcessType::General => 1,
            StochasticProcessType::Markov => 2,
            StochasticProcessType::Martingale => 3,
            StochasticProcessType::BrownianMotion => 3,
            StochasticProcessType::PoissonProcess => 2,
            StochasticProcessType::RandomWalk => 2,
        };

        1 + process_complexity
            + self.core.complexity()
            + self.index_set.complexity()
            + self.state_space.complexity()
    }
}

impl Complexity for MarkovChain {
    fn complexity(&self) -> usize {
        let transition_complexity = match &self.transition_matrix {
            TransitionMatrix::Finite(matrix) => {
                matrix.len() * matrix.get(0).map_or(0, |row| row.len())
            }
            TransitionMatrix::Kernel(_) => 5, // Kernel is complex
        };

        1 + self.core.complexity()
            + self.state_space.complexity()
            + self.initial_distribution.complexity()
            + transition_complexity
    }
}

impl Complexity for Martingale {
    fn complexity(&self) -> usize {
        1 + self.core.complexity() + 2 // Filtration adds complexity
    }
}

impl Complexity for BrownianMotion {
    fn complexity(&self) -> usize {
        todo!()
    }
}

impl Complexity for ProbabilityRelation {
    fn complexity(&self) -> usize {
        match self {
            ProbabilityRelation::EventsAreIndependent { events, .. } => 2 + events.len(),
            ProbabilityRelation::RandomVariablesAreIndependent { variables, .. } => {
                2 + variables.len()
            }
            ProbabilityRelation::HasDistribution { .. } => 2,
            ProbabilityRelation::IdenticallyDistributed { variables } => 2 + variables.len(),
            ProbabilityRelation::ConvergesTo { sequence, .. } => 3 + sequence.len(),
            ProbabilityRelation::ConditionallyIndependent { variables, .. } => 3 + variables.len(),
            ProbabilityRelation::SatisfiesMarkovProperty { .. } => 3,
            ProbabilityRelation::IsMartingale { .. } => 3,
            ProbabilityRelation::EventHasProbability { .. } => 1,
            ProbabilityRelation::HasExpectedValue { .. } => 2,
            ProbabilityRelation::HasVariance { .. } => 2,
            ProbabilityRelation::SatisfiesLawOfLargeNumbers { sequence, .. } => 4 + sequence.len(),
            ProbabilityRelation::SatisfiesCentralLimitTheorem { sequence } => 4 + sequence.len(),
            ProbabilityRelation::ConditionalExpectation { .. } => 4,
        }
    }
}

impl Complexity for ProbabilityExpression {
    fn complexity(&self) -> usize {
        match self {
            ProbabilityExpression::EventProbability { .. } => 1,
            ProbabilityExpression::ConditionalProbability { .. } => 2,
            ProbabilityExpression::ExpectedValue { .. } => 2,
            ProbabilityExpression::ConditionalExpectedValue { .. } => 3,
            ProbabilityExpression::Variance { .. } => 2,
            ProbabilityExpression::Covariance { .. } => 3,
            ProbabilityExpression::Moment { .. } => 2,
            ProbabilityExpression::CharacteristicFunction { .. } => 3,
            ProbabilityExpression::MomentGeneratingFunction { .. } => 3,
            ProbabilityExpression::RandomVariableSum { .. } => 2,
            ProbabilityExpression::RandomVariableProduct { .. } => 2,
            ProbabilityExpression::EventUnion { .. } => 1,
            ProbabilityExpression::EventIntersection { .. } => 1,
            ProbabilityExpression::EventComplement { .. } => 1,
            ProbabilityExpression::IndicatorVariable { .. } => 1,
            ProbabilityExpression::DistributionFunction { .. } => 2,
            ProbabilityExpression::ProbabilityDensityFunction { .. } => 3,
            ProbabilityExpression::ProbabilityMassFunction { .. } => 2,
        }
    }
}
