fn main() {
    let result = fib(9);
    println!("Fibonacci nth number: {result}");
}

fn fib(nth: i32) -> i32 {
    let mut prev = 0;
    let mut curr = 1;
    let mut res = 0;

    let mut i = 2;

    while i < nth {
        res = prev + curr;
        prev = curr;
        curr = res;
        
        i += 1;
    }

    return res;
}
