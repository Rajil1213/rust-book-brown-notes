mod borrow_checker;
mod dereferences;
mod ownership;
mod references_and_borrowing;
mod slices;

fn main() {
    ownership::test();
    println!("------------------------------");
    references_and_borrowing::test();
    println!("------------------------------");
    dereferences::test();
    println!("------------------------------");
    slices::test();
    println!("------------------------------");
    borrow_checker::test();
}
