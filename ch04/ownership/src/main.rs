fn main() {
    let s = String::from("hello");

    let mut mutable_s = String::from("hello");

    mutable_s.push_str(", world!"); // push_str() appends a literal to a string
    
    println!("{}", s); // Prints only `hello`
    println!("{}", mutable_s); // This prints `hello, world!`
    
    let s1 = String::from("hello");
    let s2 = s1; // Moves ownership to s2 variable.

    // println!("{}, world!", s1); this would result in an error because s1 goes
    // out of scope when moved to s2
    println!("{}, world!", s2);

    // Variables and Data Interacting with Clone
    let str1 = String::from("hello");
    let str1_cloned = str1.clone();

    println!("str1 = {}, str1_cloned = {}", str1, str1_cloned);

    let x = 5;
    let y = x;

    // This works even though we didn't call clone to x because types like
    // integers work have known size at compile time are stored entirely on stack
    println!("x = {}, y = {}", x, y); 

    // Ownership and Functions
    let str = String::from("hello"); // str comes in to scope

    takes_ownership(str);            // str's value moves into the function...
                                     // ... and so is no longer valid here 

    let z = 5;                       // x would move into the function,
                                     // but i32 is Copy, so it's ok to still
                                     // use x afterwards

    makes_copy(z);

    let new_str1 = gives_ownership();    // gives_ownership moves its return
                                         // value into new_str1
    let new_str2 = String::from("hello"); // new_str2 comes into scope

    let new_str3 = takes_and_gives_back(new_str2); // new_str2 is moved into
                                                   // takes_and_gives_back, which also
                                                   // moves its return value into new_str3

    let hello_str = String::from("hello");

    let (hello_str2, len) = calculate_length(hello_str);

    println!("The length of '{}' is {}.", hello_str2, len);
} // Here, new_str3 goes out of scope and is dropped. new_str2 was moved, so nothing
// happens. new_str1 goes out of scope and is dropped.

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {            // gives_ownership will move its
                                            // return value into the function
                                            // that calls it 
    let some_string = String::from("yours"); // some_string comes into the scope

    some_string                             // some_string is returned and
                                            // moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {   // a_string comes into scope 
    a_string    // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    
    (s, length)
}
