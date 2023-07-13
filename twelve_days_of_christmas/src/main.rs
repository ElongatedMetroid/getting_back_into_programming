fn main() {
    let days = 
        ["first", "second", "third", "fourth",
        "fifth", "sixth", "seventh", "eighth",
        "ninth", "tenth", "eleventh", "twelfth"];

    let gifts = 
        ["Twelve drummers drumming", 
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves, and",
        "A partridge in a pear tree"];
    
    for i in 0..12 {
        println!("On the {} day of Christmas, my true love sent to me", days[i]);
        for gift in gifts.iter().skip(11 - i) {
            println!("{gift}");
        }
        println!()
    }
}
