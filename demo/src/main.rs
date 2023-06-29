mod ownership;
mod references_and_borrowing;
mod slices;

fn main() {
    ownership::test();
    println!("------------------------------");
    references_and_borrowing::test();
    println!("------------------------------");
    slices::test();
}
