#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Ident(String),
}

impl Value {
    pub fn add(self, other: Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => Some(Value::Number(left + right)),
            (Value::String(left), Value::String(right)) => {
                Some(Value::String(left.to_owned() + right.as_str()))
            }
            _ => None,
        }
    }

    pub fn sub(self, other: Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => Some(Value::Number(left - right)),
            _ => None,
        }
    }

    pub fn mul(self, other: Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => Some(Value::Number(left * right)),
            _ => None,
        }
    }

    pub fn div(self, other: Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => Some(Value::Number(left / right)),
            _ => None,
        }
    }
}
