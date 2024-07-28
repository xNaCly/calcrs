#[derive(Debug, Clone)]
pub enum Type {
    Number(f64),
    String(String),
}

impl Type {
    pub fn add(self, other: Type) -> Option<Type> {
        match (self, other) {
            (Type::Number(left), Type::Number(right)) => Some(Type::Number(left + right)),
            (Type::String(left), Type::String(right)) => {
                Some(Type::String(left.to_owned() + right.as_str()))
            }
            _ => None,
        }
    }

    pub fn sub(self, other: Type) -> Option<Type> {
        match (self, other) {
            (Type::Number(left), Type::Number(right)) => Some(Type::Number(left - right)),
            _ => None,
        }
    }

    pub fn mul(self, other: Type) -> Option<Type> {
        match (self, other) {
            (Type::Number(left), Type::Number(right)) => Some(Type::Number(left * right)),
            _ => None,
        }
    }

    pub fn div(self, other: Type) -> Option<Type> {
        match (self, other) {
            (Type::Number(left), Type::Number(right)) => Some(Type::Number(left / right)),
            _ => None,
        }
    }
}
