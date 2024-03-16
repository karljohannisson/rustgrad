use std::{fmt, ops};

pub struct Value {
    data: f64,
    children: Option<Box<Value>>,
}

impl Value {
    pub fn new<'a>(data: f64, children: Option<Box<Value>>) -> Value {
        Value {
            data,
            children,
        }
    }
}

impl ops::Add<Value> for Value {
    type Output = Self;
    fn add(self, _rhs: Value) -> Self {
        Self {
            data: self.data + &_rhs.data,
            children: Some(Box::new(_rhs)),
        }
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

    //#[test]
    //fn test_add() {
    //    let mut a = Value::new(10f64, None);
    //    let mut b = Value::new(3f64, None);
    //    let c=&a+&b;
    //    assert_eq!(c.data, 13f64);
    //}

    #[test]
    fn test_children() {
        let a = Value::new(10f64, None);
        let b = Value::new(3f64, None);
        let c=&a+&b;
        assert_eq!(&c.children.unwrap().data, &a.data);
    }

}
