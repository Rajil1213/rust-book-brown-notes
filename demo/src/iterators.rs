pub fn test() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // create and store iterator

    for val in v1_iter {
        // loop through the values in the iterator
        println!("Got: {val}");
    }

    let v2_iter = v1.iter().map(|x| x + 1);
    for val in v2_iter {
        println!("In new iterator, got: {val}");
    }
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let sum: i32 = v1_iter.sum();
    assert_eq!(sum, 6);
}

#[test]
fn adatper_iterator() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter().map(|x| x + 1);
    let v2_values: Vec<i32> = v1_iter.collect();
    assert_eq!(v2_values, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter()
        .filter(|shoe| shoe.size == shoe_size)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 12,
                style: String::from("sandal"),
            },
            Shoe {
                size: 12,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 12);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 12,
                    style: String::from("sandal"),
                },
                Shoe {
                    size: 12,
                    style: String::from("boot")
                }
            ]
        )
    }
}
