/// Modal operators for mathematical necessity and possibility
#[derive(Debug, Clone)]
pub enum ModalOperator {
    /// Necessity operator (□P: P is necessarily true)
    Necessary(Box<ModalOperator>),
    /// Possibility operator (◇P: P is possibly true)
    Possible(Box<ModalOperator>),
    /// No modal operator
    None,
}

impl ModalOperator {
    /// Create a necessity operator
    pub fn necessary(inner: Option<ModalOperator>) -> Self {
        match inner {
            Some(op) => ModalOperator::Necessary(Box::new(op)),
            None => ModalOperator::Necessary(Box::new(ModalOperator::None)),
        }
    }

    /// Create a possibility operator
    pub fn possible(inner: Option<ModalOperator>) -> Self {
        match inner {
            Some(op) => ModalOperator::Possible(Box::new(op)),
            None => ModalOperator::Possible(Box::new(ModalOperator::None)),
        }
    }

    /// Check if this operator contains necessity
    pub fn is_necessary(&self) -> bool {
        matches!(self, ModalOperator::Necessary(_))
    }

    /// Check if this operator contains possibility
    pub fn is_possible(&self) -> bool {
        matches!(self, ModalOperator::Possible(_))
    }
}
