// Convert strings to pig latin. The first consonant of each word is moved to the end of the word 
// and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added 
// to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 
// encoding!

fn main() {
    let s = String::from("first");

    let s_pig = to_piglatin(s);

    println!("{s_pig}");

    let s_with_vowel = String::from("apple");

    let s_vowel_piglatin = to_piglatin(s_with_vowel);

    println!("{s_vowel_piglatin}");
}

fn to_piglatin(s: String) -> String {
    println!("{s}");

    let mut piglatin_string = String::from("");
    let mut ay_str = String::from("");
    let mut i = 0;

    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'y'];

    for c in s.chars() {
        if i == 0 {
            if vowels.contains(&c) {
                ay_str = String::from("-hay");
                piglatin_string.push(c);
            } else {
                ay_str.push_str("-");
                ay_str.push(c);
                ay_str.push_str("ay");
            }
            i += 1;
        } else {
            piglatin_string.push(c);
            i += 1;
        }
    }
    piglatin_string.push_str(&ay_str);
    piglatin_string
}
