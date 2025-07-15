#[cfg(test)]
mod tests {
    use crate::subjects::math::formalism::abstraction_level::{
        AbstractionLevel, GetAbstractionLevel,
    };
    use crate::subjects::math::theories::probability::checker::*;
    use crate::subjects::math::theories::probability::definitions::*;
    use crate::turn_render::Identifier;
    use serde_json::Number;

    #[test]
    fn test_generic_probability_space_creation() {
        let space = GenericProbabilitySpace::default();
        assert_eq!(space.level(), AbstractionLevel::Level1);
    }

    #[test]
    fn test_discrete_probability_space_validation() {
        let mut space = DiscreteProbabilitySpace {
            core: GenericProbabilitySpace::default(),
            sample_points: vec!["head".to_string(), "tail".to_string()],
            point_probabilities: std::collections::HashMap::new(),
            discrete_props: crate::subjects::math::theories::VariantSet::new(),
        };

        // Should fail without probabilities
        assert!(validate_discrete_probability_space(&space).is_err());

        // Add probabilities
        space.point_probabilities.insert(
            Identifier::new_simple("head".to_string()),
            Number::from_f64(0.5).unwrap(),
        );
        space.point_probabilities.insert(
            Identifier::new_simple("tail".to_string()),
            Number::from_f64(0.5).unwrap(),
        );

        // Should pass now
        assert!(validate_discrete_probability_space(&space).is_ok());
    }

    #[test]
    fn test_probability_space_abstraction_levels() {
        let generic_space = ProbabilitySpace::Generic(GenericProbabilitySpace::default());
        assert_eq!(generic_space.level(), AbstractionLevel::Level1);

        let discrete_space = ProbabilitySpace::Discrete(DiscreteProbabilitySpace {
            core: GenericProbabilitySpace::default(),
            sample_points: vec!["1".to_string()],
            point_probabilities: [(
                Identifier::new_simple("1".to_string()),
                Number::from_f64(1.0).unwrap(),
            )]
            .iter()
            .cloned()
            .collect(),
            discrete_props: crate::subjects::math::theories::VariantSet::new(),
        });
        assert_eq!(discrete_space.level(), AbstractionLevel::Level3);
    }

    #[test]
    fn test_markov_chain_validation() {
        let transition_matrix = TransitionMatrix::Finite(vec![
            vec![
                Number::from_f64(0.7).unwrap(),
                Number::from_f64(0.3).unwrap(),
            ],
            vec![
                Number::from_f64(0.4).unwrap(),
                Number::from_f64(0.6).unwrap(),
            ],
        ]);

        let chain = MarkovChain {
            core: StochasticProcess {
                core: GenericProbabilitySpace::default(),
                index_set: crate::subjects::math::theories::zfc::definitions::Set::Parametric {
                    parameters: std::collections::HashMap::new(),
                    description: "Natural numbers".to_string(),
                    membership_condition: "n ∈ ℕ".to_string(),
                    properties: crate::subjects::math::theories::VariantSet::new(),
                },
                state_space: crate::subjects::math::theories::zfc::definitions::Set::Parametric {
                    parameters: std::collections::HashMap::new(),
                    description: "Binary states".to_string(),
                    membership_condition: "s ∈ {0,1}".to_string(),
                    properties: crate::subjects::math::theories::VariantSet::new(),
                },
                process_type: StochasticProcessType::Markov,
                process_props: crate::subjects::math::theories::VariantSet::new(),
            },
            state_space: crate::subjects::math::theories::zfc::definitions::Set::Parametric {
                parameters: std::collections::HashMap::new(),
                description: "Binary states".to_string(),
                membership_condition: "s ∈ {0,1}".to_string(),
                properties: crate::subjects::math::theories::VariantSet::new(),
            },
            transition_matrix,
            initial_distribution: Distribution {
                random_variable: Box::new(RandomVariable {
                    probability_space: Box::new(ProbabilitySpace::Generic(
                        GenericProbabilitySpace::default(),
                    )),
                    target_space:
                        crate::subjects::math::theories::zfc::definitions::Set::Parametric {
                            parameters: std::collections::HashMap::new(),
                            description: "Real numbers".to_string(),
                            membership_condition: "x ∈ ℝ".to_string(),
                            properties: crate::subjects::math::theories::VariantSet::new(),
                        },
                    variable_type: RandomVariableType::Discrete,
                    props: crate::subjects::math::theories::VariantSet::new(),
                }),
                distribution_type: DistributionType::Discrete(
                    DiscreteDistributionVariant::Bernoulli,
                ),
                parameters: DistributionParameters {
                    parameters: [(
                        Identifier::new_simple("p".to_string()),
                        Number::from_f64(0.5).unwrap(),
                    )]
                    .iter()
                    .cloned()
                    .collect(),
                    constraints: vec![ParameterConstraint::Probability("p".to_string())],
                },
                props: crate::subjects::math::theories::VariantSet::new(),
            },
            markov_props: crate::subjects::math::theories::VariantSet::new(),
        };

        assert!(validate_markov_chain(&chain).is_ok());
    }

    #[test]
    fn test_distribution_parameter_validation() {
        let mut params = DistributionParameters {
            parameters: std::collections::HashMap::new(),
            constraints: vec![ParameterConstraint::Probability("p".to_string())],
        };

        // Invalid probability
        params.parameters.insert(
            Identifier::new_simple("p".to_string()),
            Number::from_f64(1.5).unwrap(),
        );
        let dist = Distribution {
            random_variable: Box::new(RandomVariable {
                probability_space: Box::new(ProbabilitySpace::Generic(
                    GenericProbabilitySpace::default(),
                )),
                target_space: crate::subjects::math::theories::zfc::definitions::Set::Parametric {
                    parameters: std::collections::HashMap::new(),
                    description: "Real numbers".to_string(),
                    membership_condition: "x ∈ ℝ".to_string(),
                    properties: crate::subjects::math::theories::VariantSet::new(),
                },
                variable_type: RandomVariableType::Discrete,
                props: crate::subjects::math::theories::VariantSet::new(),
            }),
            distribution_type: DistributionType::Discrete(DiscreteDistributionVariant::Bernoulli),
            parameters: params.clone(),
            props: crate::subjects::math::theories::VariantSet::new(),
        };

        assert!(validate_distribution(&dist).is_err());

        // Valid probability
        params.parameters.insert(
            Identifier::new_simple("p".to_string()),
            Number::from_f64(0.3).unwrap(),
        );
        let valid_dist = Distribution {
            parameters: params,
            ..dist
        };
        assert!(validate_distribution(&valid_dist).is_ok());
    }
}
