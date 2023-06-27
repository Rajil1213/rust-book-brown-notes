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

        let mut verse: String = String::from(first_line.to_owned() + "\n");

        // go over the lines in reverse order for each line `i`
        for j in (0..=i).rev() {
            verse.push_str(LINES[j]);
            verse.push_str("\n");
        }

        lyrics.push_str(&verse.to_owned());
        lyrics.push_str("\n")
    }

    print!("{}", lyrics);
}
