mod value;

use crate::value::Value;


fn main() {
    let mut a = Value::new(10f64, None);
    let mut b = Value::new(3f64, None);

    println!("value data is: {}", a + b)
}
