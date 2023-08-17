use std::{cell::RefCell, rc::Rc};

// only allow single owner
#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

// allow multiple owners
#[derive(Debug)]
pub enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

// allow multiple owners, allow mutation
#[derive(Debug)]
pub enum RefCellList {
    Cons(Rc<RefCell<i32>>, Rc<RefCellList>),
    Nil,
}

#[derive(Debug)]
pub enum UnsafeList {
    Cons(i32, RefCell<Rc<UnsafeList>>),
    Nil,
}

impl UnsafeList {
    pub fn tail(&self) -> Option<&RefCell<Rc<UnsafeList>>> {
        match self {
            Self::Cons(_value, item) => Some(item),
            Self::Nil => None,
        }
    }
}
