use crate::subjects::math::formalism::{
    expressions::{Identifier, MathExpression},
    proof::{ProofForest, ProofGoal},
    relations::{MathRelation, RelationDetail},
    theorem::Theorem,
};

/// Returns a list of all theorems in the probability theory module.
pub fn all_probability_theorems() -> Vec<Theorem> {
    vec![
        prove_weak_law_of_large_numbers(),
        prove_strong_law_of_large_numbers(),
        prove_central_limit_theorem(),
        prove_bayes_theorem(),
        prove_law_of_total_probability(),
        prove_chebyshev_inequality(),
        prove_markov_inequality(),
        prove_jensen_inequality(),
        prove_martingale_convergence_theorem(),
        prove_optional_stopping_theorem(),
        prove_kolmogorov_three_series_theorem(),
        prove_glivenko_cantelli_theorem(),
    ]
}

// --- Individual Theorem Definitions ---

fn prove_weak_law_of_large_numbers() -> Theorem {
    Theorem {
        id: "probability.weak_law_of_large_numbers".to_string(),
        name: "Weak Law of Large Numbers".to_string(),
        description: "For independent identically distributed random variables with finite expectation, the sample mean converges in probability to the population mean.".to_string(),
        proofs: ProofForest::new_from_goal(ProofGoal {
            statement: MathRelation::Todo {
            name: "converges_in_probability".to_string(),
            expressions: vec![
                MathExpression::Var(Identifier::Name("sample_mean".to_string(), 1)),
                MathExpression::Var(Identifier::Name("population_mean".to_string(), 2)),
            ],
            },
            quantifiers: vec![],
            value_variables: vec![],
        }),
    }
}

fn prove_strong_law_of_large_numbers() -> Theorem {
    Theorem {
        id: "probability.strong_law_of_large_numbers".to_string(),
        name: "Strong Law of Large Numbers".to_string(),
        description: "For independent identically distributed random variables with finite expectation, the sample mean converges almost surely to the population mean.".to_string(),
        proofs: ProofForest::new_from_goal(ProofGoal {
            statement: MathRelation::Todo {
            name: "converges_almost_surely".to_string(),
            expressions: vec![
                MathExpression::Var(Identifier::Name("sample_mean".to_string(), 1)),
                MathExpression::Var(Identifier::Name("population_mean".to_string(), 2)),
            ],
            },
            quantifiers: vec![],
            value_variables: vec![],
        }),
    }
}

fn prove_central_limit_theorem() -> Theorem {
    Theorem {
        id: "probability.central_limit_theorem".to_string(),
        name: "Central Limit Theorem".to_string(),
        description: "For independent identically distributed random variables with finite second moment, the standardized sum converges in distribution to a standard normal distribution.".to_string(),
        proofs: ProofForest::new_from_goal(ProofGoal {
            statement: MathRelation::Todo {
            name: "converges_in_distribution".to_string(),
            expressions: vec![
                MathExpression::Var(Identifier::Name("standardized_sum".to_string(), 1)),
                MathExpression::Var(Identifier::Name("standard_normal".to_string(), 2)),
            ],
            },
            quantifiers: vec![],
            value_variables: vec![],
        }),
    }
}

fn prove_bayes_theorem() -> Theorem {
    Theorem {
        id: "probability.bayes_theorem".to_string(),
        name: "Bayes' Theorem".to_string(),
        description: "For events A and B with P(B) > 0, P(A|B) = P(B|A)P(A)/P(B).".to_string(),
        proofs: ProofForest::new_from_goal(ProofGoal {
            statement: MathRelation::Equal {
                left: MathExpression::Var(Identifier::Name("P(A|B)".to_string(), 1)),
                right: MathExpression::Var(Identifier::Name("P(B|A)P(A)/P(B)".to_string(), 2)),
                meta: RelationDetail {
                    expressions: vec![
                        MathExpression::Var(Identifier::Name("P(A|B)".to_string(), 1)),
                        MathExpression::Var(Identifier::Name("P(B|A)P(A)/P(B)".to_string(), 2)),
                    ],
                    metadata: std::collections::HashMap::new(),
                    description: Some("Bayes' theorem equality".to_string()),
                    is_reflexive: false,
                    is_symmetric: false,
                },
            },
            quantifiers: vec![],
            value_variables: vec![],
        }),
    }
}

fn prove_law_of_total_probability() -> Theorem {
    Theorem {
        id: "probability.law_of_total_probability".to_string(),
        name: "Law of Total Probability".to_string(),
        description: "For a partition of the sample space {B_i}, P(A) = Σ P(A|B_i)P(B_i)."
            .to_string(),
        proofs: ProofForest::new_from_goal(ProofGoal {
            statement: MathRelation::Equal {
                left: MathExpression::Var(Identifier::Name("P(A)".to_string(), 1)),
                right: MathExpression::Var(Identifier::Name("Σ P(A|B_i)P(B_i)".to_string(), 2)),
                meta: RelationDetail {
                    expressions: vec![
                        MathExpression::Var(Identifier::Name("P(A)".to_string(), 1)),
                        MathExpression::Var(Identifier::Name("Σ P(A|B_i)P(B_i)".to_string(), 2)),
                    ],
                    metadata: std::collections::HashMap::new(),
                    description: Some("Law of total probability".to_string()),
                    is_reflexive: false,
                    is_symmetric: false,
                },
            },
            quantifiers: vec![],
            value_variables: vec![],
        }),
    }
}

fn prove_chebyshev_inequality() -> Theorem {
    Theorem {
        id: "probability.chebyshev_inequality".to_string(),
        name: "Chebyshev's Inequality".to_string(),
        description: "For a random variable X with finite variance σ², P(|X - μ| ≥ kσ) ≤ 1/k²."
            .to_string(),
        proofs: ProofForest::new_from_goal(ProofGoal {
            statement: MathRelation::Todo {
                name: "probability_bound".to_string(),
                expressions: vec![
                    MathExpression::Var(Identifier::Name("P(|X - μ| ≥ kσ)".to_string(), 1)),
                    MathExpression::Var(Identifier::Name("1/k²".to_string(), 2)),
                ],
            },
            quantifiers: vec![],
            value_variables: vec![],
        }),
    }
}

fn prove_markov_inequality() -> Theorem {
    Theorem {
        id: "probability.markov_inequality".to_string(),
        name: "Markov's Inequality".to_string(),
        description:
            "For a non-negative random variable X with finite expectation, P(X ≥ a) ≤ E[X]/a."
                .to_string(),
        proofs: ProofForest::new_from_goal(ProofGoal {
            statement: MathRelation::Todo {
                name: "probability_bound".to_string(),
                expressions: vec![
                    MathExpression::Var(Identifier::Name("P(X ≥ a)".to_string(), 1)),
                    MathExpression::Var(Identifier::Name("E[X]/a".to_string(), 2)),
                ],
            },
            quantifiers: vec![],
            value_variables: vec![],
        }),
    }
}

fn prove_jensen_inequality() -> Theorem {
    Theorem {
        id: "probability.jensen_inequality".to_string(),
        name: "Jensen's Inequality".to_string(),
        description: "For a convex function φ and integrable random variable X, φ(E[X]) ≤ E[φ(X)]."
            .to_string(),
        proofs: ProofForest::new_from_goal(ProofGoal {
            statement: MathRelation::Todo {
                name: "inequality".to_string(),
                expressions: vec![
                    MathExpression::Var(Identifier::Name("φ(E[X])".to_string(), 1)),
                    MathExpression::Var(Identifier::Name("E[φ(X)]".to_string(), 2)),
                ],
            },
            quantifiers: vec![],
            value_variables: vec![],
        }),
    }
}

fn prove_martingale_convergence_theorem() -> Theorem {
    Theorem {
        id: "probability.martingale_convergence".to_string(),
        name: "Martingale Convergence Theorem".to_string(),
        description: "A martingale bounded in L¹ converges almost surely.".to_string(),
        proofs: ProofForest::new_from_goal(ProofGoal {
            statement: MathRelation::Todo {
                name: "converges_almost_surely".to_string(),
                expressions: vec![
                    MathExpression::Var(Identifier::Name("X_n".to_string(), 1)),
                    MathExpression::Var(Identifier::Name("X_∞".to_string(), 2)),
                ],
            },
            quantifiers: vec![],
            value_variables: vec![],
        }),
    }
}

fn prove_optional_stopping_theorem() -> Theorem {
    Theorem {
        id: "probability.optional_stopping".to_string(),
        name: "Optional Stopping Theorem".to_string(),
        description: "For a martingale and stopping time satisfying integrability conditions, E[X_τ] = E[X_0].".to_string(),
        proofs: ProofForest::new_from_goal(ProofGoal {
            statement: MathRelation::Equal {
                left: MathExpression::Var(Identifier::Name("E[X_τ]".to_string(), 1)),
                right: MathExpression::Var(Identifier::Name("E[X_0]".to_string(), 2)),
                meta: RelationDetail {
                    expressions: vec![
                        MathExpression::Var(Identifier::Name("E[X_τ]".to_string(), 1)),
                        MathExpression::Var(Identifier::Name("E[X_0]".to_string(), 2)),
                    ],
                    metadata: std::collections::HashMap::new(),
                    description: Some("Optional stopping theorem equality".to_string()),
                    is_reflexive: false,
                    is_symmetric: false,
                },
            },
            quantifiers: vec![],
            value_variables: vec![],
        }),
    }
}

fn prove_kolmogorov_three_series_theorem() -> Theorem {
    Theorem {
        id: "probability.kolmogorov_three_series".to_string(),
        name: "Kolmogorov's Three-Series Theorem".to_string(),
        description: "For independent random variables, the series Σ X_n converges almost surely if and only if three associated series converge.".to_string(),
        proofs: ProofForest::new_from_goal(ProofGoal {
            statement: MathRelation::Todo {
            name: "series_convergence_equivalence".to_string(),
            expressions: vec![
                MathExpression::Var(Identifier::Name("Σ X_n converges a.s.".to_string(), 1)),
                MathExpression::Var(Identifier::Name("Three series converge".to_string(), 2)),
            ],
            },
            quantifiers: vec![],
            value_variables: vec![],
        }),
    }
}

fn prove_glivenko_cantelli_theorem() -> Theorem {
    Theorem {
        id: "probability.glivenko_cantelli".to_string(),
        name: "Glivenko-Cantelli Theorem".to_string(),
        description: "The empirical distribution function converges uniformly to the true distribution function almost surely.".to_string(),
        proofs: ProofForest::new_from_goal(ProofGoal {
            statement: MathRelation::Todo {
            name: "uniform_convergence".to_string(),
            expressions: vec![
                MathExpression::Var(Identifier::Name("F_n(x)".to_string(), 1)),
                MathExpression::Var(Identifier::Name("F(x)".to_string(), 2)),
            ],
            },
            quantifiers: vec![],
            value_variables: vec![],
        }),
    }
}
