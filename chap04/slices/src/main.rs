fn main() {
    let words = String::from("Hello world");

    println!("Words is: {}", words);

    println!("{}", first_word(&words[..]));

    let my_string = String::from("Hello World");

    // first_word works on slices of 'String's
    let word = first_word(&my_string[..]);

    println!("word: {}", word);

    let my_string_literal = "hello world";
    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);
    println!("word now: {}", word);

    // since string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("last word: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}