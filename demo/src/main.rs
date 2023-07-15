mod enums;
mod functions;
mod structs;

fn main() {
    let num_list = vec![17, 31, 23, 13];

    let largest = functions::get_largest(&num_list);
    println!("The largest number in {num_list:?} is {largest}");

    let int_point = structs::Point { x: 5, y: 10 };
    let float_point = structs::Point { x: 1.0, y: 2.0 };

    let exists = enums::Optional::Exists(2);
    let does_not_exist: enums::Optional<i32> = enums::Optional::None;
}
