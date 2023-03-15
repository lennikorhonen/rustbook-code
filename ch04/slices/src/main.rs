fn main() {
    let my_string = String::from("hello world");

    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);

    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    let word = first_word(&my_string_literal);

    // s.clear(); error would cause error because can't make immutable and mutable references
    
    println!("the first word is: {}", word);

    let s2 = String::from("hello world");

    let hello = &s2[0..5];
    let word = &s2[6..11];
 
    let s3 = String::from("hello");
    let slice1 = &s3[0..2];
    let slice2 = &s3[..2]; // slice1 and slice2 are equal
    
    let len = s3.len();
    let slice3 = &s3[3..len];
    let slice4 = &s3[3..]; // slice3 and slice4 are also equal

    let slice5 = &s3[0..len];
    let slice6 = &s3[..]; // slice5 and slice6 are also equal
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
