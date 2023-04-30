pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v2: Vec<i32> = vec![1, 2, 3];

    let v3: Vec<_> = v2.iter().map(|x| x + 1).collect();

    assert_eq!(v3, vec![2, 3, 4]);
}
