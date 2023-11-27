fn main() {
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French Hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for day in 1..=12 {
        // let mut current= 1;
        let suffix = if day == 1 {
            "st"
        } else if day == 2 {
            "nd"
        } else if day == 3 {
            "rd"
        } else {
            "th"
        };
        println!("On the {day}{suffix} day of Christmas, My true love gave to me: ");
        for x in (0..day).rev() {
            
            println!("{}", gifts[x]);
            if x == 1 {
                println! {"and"}
            }
        }

        println!("------------------")
    }
}
