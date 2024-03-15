use std::{fmt, ops};

pub struct Value {
    data: f64,
}

impl Value {
    pub fn new(data: f64) -> Value {
        Value {
            data: data,
        }
    }
}

impl ops::Add<Value> for Value {
    type Output = Value;
    fn add(self, _rhs: Value) -> Value {
        Value::new(self.data + _rhs.data)
    }
}

impl ops::Mul<Value> for Value {
    type Output = Value;
    fn mul(self, _rhs: Value) -> Value {
        Value::new(self.data * _rhs.data)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value={}", self.data)
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let mut a = Value::new(10f64);
        let mut b = Value::new(3f64);
        let c=a+b;
        assert_eq!(c.data, 13f64);
    }

    #[test]
    fn test_mul() {
        let mut a = Value::new(10f64);
        let mut b = Value::new(3f64);
        let c=a*b;
        assert_eq!(c.data, 30f64);
    }

}