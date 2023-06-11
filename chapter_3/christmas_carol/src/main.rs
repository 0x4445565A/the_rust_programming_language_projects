fn human_readable_day(n: i32) -> &'static str {
    match n {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "nineth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "scariest",
    }
}

fn gift_phrase(n: i32) -> &'static str {
    match n {
        1 => "A partridge in a pear tree",
        2 => "Two turtle doves, and",
        3 => "Three french hens",
        4 => "Four calling birds",
        5 => "Five golden rings",
        6 => "Six geese a-laying",
        7 => "Seven swans a-swimming",
        8 => "Eight maids a-milking",
        9 => "Nine ladies dancing",
        10 => "Ten lords a-leaping",
        11 => "Eleven pipers piping",
        12 => "Twelve drummers drumming",
        _ => "Some messed up stuff",
    }
}

fn main() {
    for i in 1..=12 {
        println!(
            "On the {} day of Christmas, my true love sent to me",
            human_readable_day(i)
        );
        for ii in (1..=i).rev() {
            println!("{}", gift_phrase(ii));
        }
        println!("---")
    }
}
