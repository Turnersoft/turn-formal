use crate::{
    subjects::math::formalism::{expressions::Identifier, extract::Parametrizable},
    turn_render::{MathNode, MathNodeContent, ToTurnMath},
};

impl<T: ToTurnMath> ToTurnMath for Parametrizable<T> {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            Parametrizable::Concrete(object) => object.to_turn_math(master_id),
            Parametrizable::Variable(identifier) => identifier.to_turn_math(master_id),
        }
    }
}

impl ToTurnMath for Identifier {
    fn to_turn_math(&self, master_id: String) -> MathNode {
        match self {
            Identifier::O(o) => MathNode {
                id: master_id.clone(),
                content: Box::new(MathNodeContent::Identifier {
                    body: "O".to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: Some(Box::new(MathNode {
                        id: master_id,
                        content: Box::new(MathNodeContent::Text(format!("{}", o))),
                    })),
                    primes: 0,
                    is_function: false,
                }),
            },
            Identifier::M(m) => MathNode {
                id: master_id.clone(),
                content: Box::new(MathNodeContent::Identifier {
                    body: "M".to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: Some(Box::new(MathNode {
                        id: master_id,
                        content: Box::new(MathNodeContent::Text(format!("{}", m))),
                    })),
                    primes: 0,
                    is_function: false,
                }),
            },
            Identifier::E(e) => MathNode {
                id: master_id.clone(),
                content: Box::new(MathNodeContent::Identifier {
                    body: "E".to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: Some(Box::new(MathNode {
                        id: master_id,
                        content: Box::new(MathNodeContent::Text(format!("{}", e))),
                    })),
                    primes: 0,
                    is_function: false,
                }),
            },
            Identifier::N(n) => MathNode {
                id: master_id.clone(),
                content: Box::new(MathNodeContent::Identifier {
                    body: "N".to_string(),
                    pre_script: None,
                    mid_script: None,
                    post_script: Some(Box::new(MathNode {
                        id: master_id,
                        content: Box::new(MathNodeContent::Text(format!("{}", n))),
                    })),
                    primes: 0,
                    is_function: false,
                }),
            },
            Identifier::Name(name, index) => MathNode {
                id: master_id.clone(),
                content: Box::new(MathNodeContent::Identifier {
                    body: name.clone(),
                    pre_script: None,
                    mid_script: None,
                    post_script: Some(Box::new(MathNode {
                        id: master_id,
                        content: Box::new(MathNodeContent::Text(format!("{}", index))),
                    })),
                    primes: 0,
                    is_function: false,
                }),
            },
        }
    }
}
