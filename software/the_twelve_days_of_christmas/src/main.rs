fn main() {
    let gifts = [
        "Two turtle doves",
        "Three French hens", 
        "Four calling birds", 
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
        ];
    
    let days = [
        "first", 
        "second", 
        "third", 
        "fourth", 
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
        ];

    let mut count = 0;
    let mut accumulator = "".to_owned();

    for gift in gifts {
        let day = days[count];
        let mut connector = if count == 0 {
            "A"
        } else {
            "And"
        };

        println!("On the first day of Christmas,\nmy true love sent to me{accumulator}\n{connector} partridge in a pear tree.\n");

        accumulator = accumulator + "\n" + gift + ",";
        count += 1;
    }
}
