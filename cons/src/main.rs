use std::rc::Rc;

use cons::{
    List::{Cons, Nil},
    RcList::Nil as RcNil,
    RcList::{self, Cons as RcCons},
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
    let b: RcList = RcCons(3, Rc::clone(&rca));
    println!("count after creating b = {}", Rc::strong_count(&rca));
    {
        // clone rca `Rc<RcList>` again, to increase reference count to `a` from 2 to 3
        let c: RcList = RcCons(4, Rc::clone(&rca));
        println!("count after creating c = {}", Rc::strong_count(&rca));
    }
    println!(
        "count after c goes out of scope = {}",
        Rc::strong_count(&rca)
    );
}
