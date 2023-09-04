fn if_let_else(favorite_color: Option<&str>, is_tuesday: bool, age: Result<u8, ()>) -> String {
    if let Some(color) = favorite_color {
        format!("Using your favorite color: {} as your background", color)
    } else if is_tuesday {
        String::from("Tuesday is a green day")
    } else if let Ok(age) = age {
        if age > 30 {
            String::from("Using purple as the background color")
        } else {
            String::from("Using orange as the background color")
        }
    } else {
        String::from("Using blue as the background color")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn match_arms() {
        let x: Option<i32> = Some(1);
        let matched = match x {
            None => 0,
            Some(i) => i + 1,
        };

        assert!(matched > 1);
    }

    #[test]
    fn test_if_let_else() {
        struct TestCase<'a> {
            favorite_color: Option<&'a str>,
            is_tuesday: bool,
            age: Result<u8, ()>,
            expected: &'a str,
        }

        let test_cases: [TestCase; 5] = [
            TestCase {
                favorite_color: None,
                is_tuesday: false,
                age: Result::Err(()),
                expected: "Using blue as the background color",
            },
            TestCase {
                favorite_color: None,
                is_tuesday: false,
                age: Result::Ok(20),
                expected: "Using orange as the background color",
            },
            TestCase {
                favorite_color: None,
                is_tuesday: false,
                age: Result::Ok(31),
                expected: "Using purple as the background color",
            },
            TestCase {
                favorite_color: None,
                is_tuesday: true,
                age: Result::Err(()),
                expected: "Tuesday is a green day",
            },
            TestCase {
                favorite_color: Some("Black"),
                is_tuesday: false,
                age: Result::Err(()),
                expected: "Using your favorite color: Black as your background",
            },
        ];

        for test_case in test_cases {
            let result = if_let_else(
                test_case.favorite_color,
                test_case.is_tuesday,
                test_case.age,
            );
            assert_eq!(result, test_case.expected);
        }
    }

    #[test]
    fn test_while_let() {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        let expected = stack.len();

        let mut actual: usize = 0;
        while let Some(_top) = stack.pop() {
            actual += 1;
        }

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_for() {
        let v = vec!['a', 'b', 'c'];

        let mut indices = Vec::new();
        let mut values = Vec::new();
        for (index, value) in v.iter().enumerate() {
            indices.push(index);
            values.push(*value);
        }

        assert_eq!(indices, vec![0, 1, 2]);
        assert_eq!(values, v);
    }
}
