pub fn get_largest<T>(num_list: &Vec<T>) -> &T {
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
