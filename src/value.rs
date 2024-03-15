use std::{fmt, ops};
use std::rc::Rc;

pub struct Value<'a> {
    data: f64,
    children: Option<(Box<Value<'a>>, Box<Value<'a>>)>,
}

impl Value<'_> {
    pub fn new<'a>(data: f64, children: Option<(Box<Value>, Box<Value>)>) -> Value<'a> {
        Value {
            data,
            children,
        }
    }
}

impl ops::Add<Value<'_>> for Value<'_> {
    type Output<'a> = Value<'a>;
    fn add(self, _rhs: Value) -> Value {
        Value::new(self.data + _rhs.data, Some((&self, &_rhs)))
    }
}

impl ops::Mul<Value<'_>> for Value<'_> {
    type Output<'a> = Value<'a>;
    fn mul(self, _rhs: Value) -> Value {
        Value::new(self.data * _rhs.data, Some((&'a self, &'a _rhs))
    }
}

impl fmt::Display for Value<'_> {
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
        let mut a = Value::new(10f64, None);
        let mut b = Value::new(3f64, None);
        let c=a+b;
        assert_eq!(c.data, 13f64);
    }

    #[test]
    fn test_mul() {
        let mut a = Value::new(10f64, None);
        let mut b = Value::new(3f64, None);
        let c=a*b;
        assert_eq!(c.data, 30f64);
    }

    #[test]
    fn test_children() {
        let mut a = Value::new(10f64, None);
        let mut b = Value::new(3f64, None);
        let c=a*b;
        assert_eq!(c.children, Some(Rc::new((a, b))));
    }

}