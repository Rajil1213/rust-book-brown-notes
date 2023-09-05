fn matching_literals(x: i32) -> String {
    match x {
        1 => String::from("Got 1"),
        2 => String::from("Got 2"),
        _ => format!("Got {x}, which is greater than 2"),
    }
}

fn matching_named_variables(x: Option<i32>, y: i32) -> String {
    match x {
        Some(50) => String::from("Got 50"),
        // a new scope is created here,
        // where the named literal `y` is different than in the fn param
        Some(y) => format!("Matched y = {y}"),
        // here, `y` is the same as the param
        _ => format!("Default case, x = {:?}, y = {y}", x),
    }
}

fn multiple_patterns(x: i32) -> String {
    match x {
        1 | 2 => String::from("one or two"),
        3 => String::from("three"),
        _ => String::from("none of 1, 2 and 3"),
    }
}

fn matching_ranges(x: i32) -> String {
    match x {
        // equivalent to 1 | 2 | 3 | 4 | 5
        1..=5 => String::from("one through five"),
        _ => String::from("something else"),
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsla(i32, f64, f64, f64),
}

enum Message {
    Quit,
    ChangeColor(Color),
}

fn matching_nested_structs_and_enums(msg: Message) -> String {
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            format!("changing color to red {r}, green {g}, and blue {b}")
        }
        Message::ChangeColor(Color::Hsla(h, s, l, a)) => {
            format!("changing color to hue {h}, saturation {s}, lightness {l}, and alpha {a}")
        }
        _ => String::from("doing something else"),
    }
}

fn match_guards(num: Option<i32>) -> String {
    match num {
        Some(x) if x % 2 == 0 => format!("The number {x} is even"),
        Some(x) => format!("The number {x} is odd"),
        None => format!("Number not found"),
    }
}

enum Hello {
    Id { id: i32 },
}

fn at_bindings(hello: Hello) -> String {
    match hello {
        Hello::Id {
            id: id_variable @ 3..=7,
        } => format!("Found an id in range: {id_variable}"),
        Hello::Id { id: 10..=12 } => String::from("Found an id in another range"),
        Hello::Id { id } => format!("Found some other id: {id}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matching_literals() {
        struct TestCase<'a> {
            x: i32,
            expected: &'a str,
        }

        let test_cases = [
            TestCase {
                x: 1,
                expected: "Got 1",
            },
            TestCase {
                x: 2,
                expected: "Got 2",
            },
            TestCase {
                x: 4,
                expected: "Got 4, which is greater than 2",
            },
        ];

        for test_case in test_cases {
            let got = matching_literals(test_case.x);
            assert_eq!(got, test_case.expected);
        }
    }

    #[test]
    fn test_matching_named_variables() {
        struct TestCase<'a> {
            x: Option<i32>,
            y: i32,
            expected: &'a str,
        }

        let test_cases = [
            TestCase {
                x: Some(50),
                y: 10,
                expected: "Got 50",
            },
            TestCase {
                x: Some(10),
                y: 20,
                expected: "Matched y = 10",
            },
            TestCase {
                x: None,
                y: 30,
                expected: &format!("Default case, x = {:?}, y = 30", Option::<i32>::None),
            },
        ];

        for test_case in test_cases {
            let got = matching_named_variables(test_case.x, test_case.y);
            assert_eq!(got, test_case.expected);
        }
    }

    #[test]
    fn test_multiple_patterns() {
        struct TestCase<'a> {
            x: i32,
            expected: &'a str,
        }

        let test_cases = [
            TestCase {
                x: 1,
                expected: "one or two",
            },
            TestCase {
                x: 2,
                expected: "one or two",
            },
            TestCase {
                x: 3,
                expected: "three",
            },
            TestCase {
                x: 5,
                expected: "none of 1, 2 and 3",
            },
        ];

        for test_case in test_cases {
            let got = multiple_patterns(test_case.x);
            assert_eq!(got, test_case.expected);
        }
    }

    #[test]
    fn test_matching_ranges() {
        let mut counter = 1;
        while counter < 6 {
            let got = matching_ranges(counter);
            counter += 1;
            assert_eq!(got, "one through five");
        }

        let got = matching_ranges(0);
        assert_eq!(got, "something else");

        let got = matching_ranges(6);
        assert_eq!(got, "something else");
    }

    #[test]
    fn test_struct_destructuring() {
        struct Point {
            x: i32,
            y: i32,
        }
        let p = Point { x: 0, y: 2 };

        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(2, b);

        match p {
            Point { x, y: 0 } => println!("On the x axis at {x}"),
            Point { x: 0, y } => println!("On the y axis at {y}"),
            Point { x, y } => {
                println!("On neither axis: ({x}, {y})");
            }
        }
    }

    #[test]
    fn test_matching_nested_structs_and_enums() {
        struct TestCase<'a> {
            msg: Message,
            expected: &'a str,
        }

        let test_cases = [
            TestCase {
                msg: Message::ChangeColor(Color::Rgb(10, 20, 30)),
                expected: "changing color to red 10, green 20, and blue 30",
            },
            TestCase {
                msg: Message::ChangeColor(Color::Hsla(10, 0.1, 0.2, 0.3)),
                expected: "changing color to hue 10, saturation 0.1, lightness 0.2, and alpha 0.3",
            },
            TestCase {
                msg: Message::Quit,
                expected: "doing something else",
            },
        ];

        for test_case in test_cases {
            let got = matching_nested_structs_and_enums(test_case.msg);
            assert_eq!(got, test_case.expected);
        }
    }

    #[test]
    fn test_match_guards() {
        struct TestCase<'a> {
            num: Option<i32>,
            expected: &'a str,
        }

        let test_cases = [
            TestCase {
                num: Some(2),
                expected: "The number 2 is even",
            },
            TestCase {
                num: Some(1),
                expected: "The number 1 is odd",
            },
            TestCase {
                num: Option::<i32>::None,
                expected: "Number not found",
            },
        ];

        for test_case in test_cases {
            let got = match_guards(test_case.num);
            assert_eq!(got, test_case.expected);
        }
    }

    #[test]
    fn test_at_bindings() {
        struct TestCase<'a> {
            hello: Hello,
            expected: &'a str,
        }

        let test_cases = [
            TestCase {
                hello: Hello::Id { id: 4 },
                expected: "Found an id in range: 4",
            },
            TestCase {
                hello: Hello::Id { id: 11 },
                expected: "Found an id in another range",
            },
            TestCase {
                hello: Hello::Id { id: 14 },
                expected: "Found some other id: 14",
            },
        ];

        for test_case in test_cases {
            let got = at_bindings(test_case.hello);
            assert_eq!(got, test_case.expected);
        }
    }
}
