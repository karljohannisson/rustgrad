use std::fmt;
use std::ops::Add;

pub struct Value {
    data: f64,
    children: Option<Box<Value>>,
}

impl Value {
    pub fn new(data: f64, children: Option<Value>) -> Value {
        Value {
            data,
            children: match children {
                           None => None,
                           Some(children) => Some(Box::new(children)),
                              },
        }
    }
}

impl Add for Value {
    type Output = Self;
    fn add(self, _rhs: Self) -> Self {
        Value::new(self.data + &_rhs.data, Some(self))

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
        let c=a+b;
        assert_eq!(c.children.unwrap().data, 10f64);
    }

}
