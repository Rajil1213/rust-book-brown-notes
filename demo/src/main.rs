mod hash_maps;
mod strings;
mod vectors;

fn main() {
    println!("######### VECTORS #########");
    vectors::test();

    println!();
    println!("######### STRINGS #########");
    strings::test();

    println!();
    println!("######### HASH MAPS #########");
    hash_maps::test();
}
