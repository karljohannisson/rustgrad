fn main() {
    println!("hello");
}

pub struct Node {
    prev: Option<Box<Node>>,
    id: u64,
}

impl Node {
    fn new(prev: Option<Node>, id:u64) -> Node {
        Node {
            prev: match prev {
                None => None,
                Some(prev) => Some(Box::new(prev))
            },
            id
        }
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_children() {
        let a = crate::Node::new(None, 1);
        let b = crate::Node::new(Some(a), 2);

        assert_eq!(1, b.prev.unwrap().id);
    }

}
