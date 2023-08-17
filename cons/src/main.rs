use std::{cell::RefCell, rc::Rc};

use cons::{
    List::{Cons, Nil},
    RcList::Nil as RcNil,
    RcList::{self, Cons as RcCons},
    RefCellList::{self, Cons as RefCellCons, Nil as RefCellNil},
};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("List: {list:?}");

    // define a new instance of RcList
    let a: RcList = RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil))));
    // convert to Rc<T> pointing to `a`, reference count to `a` becomes 1
    let rca: Rc<RcList> = Rc::new(a);
    println!("count after creating rca = {}", Rc::strong_count(&rca));

    // clone rca `Rc<RcList>`, to increase reference count to `a` from 1 to 2
    let _b: RcList = RcCons(3, Rc::clone(&rca));
    println!("count after creating b = {}", Rc::strong_count(&rca));
    {
        // clone rca `Rc<RcList>` again, to increase reference count to `a` from 2 to 3
        let _c: RcList = RcCons(4, Rc::clone(&rca));
        println!("count after creating c = {}", Rc::strong_count(&rca));
    }
    println!(
        "count after c goes out of scope = {}",
        Rc::strong_count(&rca)
    );

    // using RefCell instead to achieve mutability along with multiple ownership
    let value = Rc::new(RefCell::new(5));

    let a: RefCellList = RefCellCons(
        Rc::clone(&value),
        Rc::new(RefCellCons(Rc::new(RefCell::new(10)), Rc::new(RefCellNil))),
    );

    // this gives 2 => `value` itself and `a`
    println!("num references to value = {}", Rc::strong_count(&value));

    let rca = Rc::new(a);
    let b = RefCellCons(Rc::new(RefCell::new(3)), Rc::clone(&rca));
    let c = RefCellCons(Rc::new(RefCell::new(4)), Rc::clone(&rca));

    // this gives 3 => `rca` itself, `b` and `c`
    println!("num references to rca = {}", Rc::strong_count(&rca));

    println!("Before:");
    println!("{rca:?}");
    println!("{b:?}");
    println!("{c:?}");

    *value.borrow_mut() += 10;
    println!("After:");
    println!("{rca:?}");
    println!("{b:?}");
    println!("{c:?}");
}
