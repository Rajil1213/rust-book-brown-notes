fn main() {
    let num_list = vec![17, 31, 23, 13];

    let largest = get_largest(&num_list);
    println!("The largest number in {num_list:?} is {largest}");
}

fn get_largest(num_list: &Vec<i32>) -> &i32 {
    if num_list.len() == 0 {
        panic!("list of numbers cannot be empty");
    }

    let mut largest = &num_list[0];

    for number in num_list {
        if number > largest {
            largest = number;
        }
    }

    largest
}
