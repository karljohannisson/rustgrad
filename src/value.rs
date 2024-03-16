use std::fmt;
use std::ops::{Add, Mul};

pub struct Value {
    data: f64,
    child_a: Option<Box<Value>>,
    child_b: Option<Box<Value>>,
    op: char,
}

impl Value {
    pub fn new(data: f64) -> Value {
        Value {
            data,
            child_a: None,
            child_b: None,
            op: ' ',

        }
    }
    pub fn new_from_op(data:f64, child_a: Option<Value>, child_b: Option<Value>, op: char)
        -> Value {
            Value {
            data,
            child_a: match child_a {
                None => None,
                Some(child_a) =>
                    Some(Box::new(child_a)),
            },
            child_b: match child_b {
                None => None,
                Some(child_b) =>
                    Some(Box::new(child_b)),
            },
            op,
        }
    }

}

impl Add for Value {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Value::new_from_op(self.data + rhs.data, Some(self), Some(rhs), '+')
    }
}
impl Mul for Value {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Value::new_from_op(self.data * rhs.data, Some(self), Some(rhs), '*')
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
       let a = Value::new(10f64);
       let b = Value::new(3f64);
       let c=a+b;
       assert_eq!(c.data, 13f64);
    }

    #[test]
    fn test_mul() {
        let a = Value::new(2.5f64);
        let b = Value::new(5f64);
        let c=a*b;
        assert_eq!(c.data, 12.5f64);
    }

    #[test]
    fn test_children() {
        let a = Value::new(10f64);
        let b = Value::new(3f64);
        let c=a+b;
        assert_eq!(c.child_a.unwrap().data, 10f64);
        assert_eq!(c.child_b.unwrap().data, 3f64);

    }

    #[test]
    fn test_add_op() {
        let a = Value::new(2.5f64);
        let b = Value::new(5f64);
        let c=a+b;
        assert_eq!(c.op, '+');
    }

    #[test]
    fn test_mul_op() {
        let a = Value::new(2.5f64);
        let b = Value::new(5f64);
        let c=a*b;
        assert_eq!(c.op, '*');
    }

}
