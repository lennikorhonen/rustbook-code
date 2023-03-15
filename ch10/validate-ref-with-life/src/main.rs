use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

fn main() {
    // Following wont compile because x goes out of scope on row 7 but we still
    // try to use it later
    // let r;                 --------+-- 'a
    //                                |
    // {                              |
    //     let x = 5;         -+-- 'b |
    //     r = &x;             |      |
    // }                      -+      |
    //                                |
    // println!("r: {}", r);          |
    //                        --------+

    let x = 5;              // ---------+-- 'b
                            //          |
    let r = &x;             // --+-- 'a |
                            //   |      |
    println!("r: {}", r);   //   |      |
                            // --+      | 
                            // ---------+

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string3 = String::from("long string is long");

    {
        let string4 = String::from("xyz");
        let result2 = longest(string3.as_str(), string4.as_str());
        println!("The longest string is {}", result2);
    }

    // println!("The longest string is {}", result2); Can't print result2 beacuse it is out of
    // scope

    let novel = String::from("Call me Ishamel. Some years ago...");
    let first_senctence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_senctence,
    };

    let s: &'static str = "I have a static lifetime.";
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
