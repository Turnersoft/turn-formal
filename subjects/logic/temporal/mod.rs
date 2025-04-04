/// Temporal operators for mathematical sequences and limits
#[derive(Debug, Clone)]
pub enum TemporalOperator {
    /// Next operator (XP: P holds in the next state)
    Next(Box<TemporalOperator>),
    /// Always operator (GP: P holds in all future states)
    Always(Box<TemporalOperator>),
    /// Eventually operator (FP: P holds in some future state)
    Eventually(Box<TemporalOperator>),
    /// Until operator (P U Q: P holds until Q holds)
    Until(Box<TemporalOperator>, Box<TemporalOperator>),
    /// No temporal operator
    None,
}

impl TemporalOperator {
    /// Create a next operator
    pub fn next(inner: Option<TemporalOperator>) -> Self {
        match inner {
            Some(op) => TemporalOperator::Next(Box::new(op)),
            None => TemporalOperator::Next(Box::new(TemporalOperator::None)),
        }
    }

    /// Create an always operator
    pub fn always(inner: Option<TemporalOperator>) -> Self {
        match inner {
            Some(op) => TemporalOperator::Always(Box::new(op)),
            None => TemporalOperator::Always(Box::new(TemporalOperator::None)),
        }
    }

    /// Create an eventually operator
    pub fn eventually(inner: Option<TemporalOperator>) -> Self {
        match inner {
            Some(op) => TemporalOperator::Eventually(Box::new(op)),
            None => TemporalOperator::Eventually(Box::new(TemporalOperator::None)),
        }
    }

    /// Create an until operator
    pub fn until(left: Option<TemporalOperator>, right: Option<TemporalOperator>) -> Self {
        TemporalOperator::Until(
            Box::new(left.unwrap_or(TemporalOperator::None)),
            Box::new(right.unwrap_or(TemporalOperator::None)),
        )
    }
}
