fn main() {
    // let mut s = String::new();
    // 
    // let data = "initial contents";
    //
    // let s2 = data.to_string();
    //
    // let s3 = "initial contents".to_string();
    //
    // let s4 = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s3 = String::from("lo");
    s3.push('l');

    let s4 = String::from("Hello ");
    let s5 = String::from("world!");
    let s6 = s4 + &s5;

    println!("s6 is {s6}");

    let str1 = String::from("tic");
    let str2 = String::from("tac");
    let str3 = String::from("toe");

    // let my_str = str1 + "-" + &str2 + "-" + &str3;

    let my_str_form = format!("{str1}-{str2}-{str3}");

    // println!("{my_str}");
    println!("{my_str_form}");

    // let hello = "Здравствуйте";

    // let slice_ukr = &hello[0..4];
    // println!("{slice_ukr}");

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}
