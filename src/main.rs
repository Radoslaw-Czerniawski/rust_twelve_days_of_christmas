const DAYS_OF_CHRISTMAS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninth", "tenth",
    "eleventh", "twelfth",
];

const GIFTS_OF_CHRISTMAS: [&str; 12] = [
    "a Partridge in a Pear Tree",
    "two Turtle Doves",
    "three French Hens",
    "four Calling Birds",
    "five Golden Rings",
    "six Geese a-Laying",
    "seven Swans a-Swimming",
    "eight Maids a-Milking",
    "nine Ladies Dancing",
    "ten Lords a-Leaping",
    "eleven Pipers Piping",
    "twelve Drummers Drumming",
];

fn print_day_of_christmas(day: usize) {
    match day {
        1..=12 => println!(
            "On the {} day of Christmas my true love gave to me:",
            DAYS_OF_CHRISTMAS[day - 1],
        ),
        _ => println!("Wrong day of Christmas passed!"),
    };
}

fn print_gift(n: usize, prefix: &str, postfix: &str) {
    println!("{}{}{}", prefix, GIFTS_OF_CHRISTMAS[n], postfix);
}

fn print_all_gifts(day: usize) {
    for gift_day in (1..=day).rev() {
        let prefix = if day != 1 && gift_day == 1 {
            "and "
        } else {
            ""
        };
        let postfix = if gift_day == 1 {
            ".\n"
        } else if gift_day != 2 {
            ","
        } else {
            ""
        };

        print_gift(gift_day - 1, &prefix, &postfix);
    }
}

fn main() {
    for day in 1..=12 {
        print_day_of_christmas(day);
        print_all_gifts(day);
    }
}
