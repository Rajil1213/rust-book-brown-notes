// run `cargo expand` on this file
fn test_vec_macro() {
    let a = vec![1, 2, 3];

    println!("The vec is: {:?}", a);
}

macro_rules! my_own_vec {
    ( $($x: expr),* ) => {
        {
            #[allow(unused_mut)]
            let mut temp_vec = Vec::new();

            $(
                temp_vec.push($x);
            )*

            temp_vec
        }
    }
}

fn test_my_own_vec() {
    #[allow(clippy::vec_init_then_push)]
    let a = my_own_vec!(1, 2, 3);

    #[allow(clippy::vec_init_then_push)]
    let b = my_own_vec!(1);

    #[allow(clippy::vec_init_then_push)]
    let c: Vec<i32> = my_own_vec!();

    println!("My own vec is: {:?}", a);
    println!("My own vec with one element is: {:?}", b);
    println!("My own vec with no element is: {:?}", c);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        test_vec_macro();
        test_my_own_vec();
    }
}
