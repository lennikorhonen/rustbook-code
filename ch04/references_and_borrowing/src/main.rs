fn main() {
    let s1 = String::from("hello");
    let mut s = String::from("hello");

    let len = calculate_length(&s1); // & represents references, they allow to refer some value
                                     // without taking ownership

    println!("The length of '{}' is {}.", s1, len);

    {
        let r1 = &mut s;
        println!("{}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    change(&mut s)
}

fn calculate_length(s: &String) -> usize { // s is a reference to a string
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
