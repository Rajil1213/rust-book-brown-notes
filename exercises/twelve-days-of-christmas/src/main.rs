/**
This macro accepts an `expr` and returns it by appending a newline to it.

The `expr` should thus be a str but does not have to be a reference to it
as the macro takes the reference to the passed expression.

Example:
```
assert_eq!(with_newline!("abc"), "abc\n");
```
*/
macro_rules! with_newline {
    ($line:expr) => {
        &format!("{}\n", &$line)
    };
}

fn main() {
    const DAY: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    const LINES: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let mut lyrics: String = String::from("");

    for i in 0..LINES.len() {
        let first_line: String =
            format!("On the {} day of Christmas my true love sent to me", DAY[i]);

        let mut verse: String = String::from(with_newline!(first_line));

        // go over the lines in reverse order for each line `i`
        for j in (0..=i).rev() {
            verse.push_str(with_newline!(LINES[j]));
        }

        lyrics.push_str(with_newline!(verse));
    }

    print!("{}", lyrics);
}
