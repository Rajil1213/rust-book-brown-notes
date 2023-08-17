use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let leaf = Rc::new(Node {
            value: 3,
            children: RefCell::new(vec![]),
        });

        let branch = Rc::new(Node {
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
    }
}
