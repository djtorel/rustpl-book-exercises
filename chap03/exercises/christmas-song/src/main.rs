fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth",
                "seventh", "eigth", "ninth", "tenth", "eleventh", "twelfth"
    ];
    let items = ["Partridge in a pear tree", "Turtle Doves", "French Hens",
        "Calling Birds", "Golden Rings", "Geese a Laying", "Swans a Swimming",
        "Maids a Milking", "Ladies Dancing", "Lords a Leaping", "Pipers Piping",
        "Drummers Drumming"
    ];
    
    for x in 0..days.len() {
        println!("On the {} day of Christmas my true love sent to me:", days[x]);
        for y in (0..x + 1).rev() {
            if y == 0 {
                println!("A {}", items[y]);
            } else {
                let num = y + 1;
                if y == 1 {
                    println!("{} {} and", num, items[y]);
                } else {
                    println!("{} {}", num, items[y]);
                }
            }
        }
        println!("");
    }
}
