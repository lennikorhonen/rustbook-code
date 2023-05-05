use std::fmt;
// use std::io::Error;

type Thunk = Box<dyn Fn() + Send + 'static>;
type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

// impl<T> Option<T> {
//     pub fn unwrap(self) -> T {
//         match self {
//             Some(val) => val,
//             None => panic!("called `Option::unwrap()` on a `None` value"),
//         }
//     }
// }


fn main() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi")); 
    let f: Thunk = Box::new(|| println!("hi"));

    // print!("forever ");

    // loop has the ! type
    // loop {
    //     print!("and ever ");
    // }
}

// fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {}
fn takes_long_type(f: Thunk) {}

// fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {}
fn returns_long_type() -> Thunk {}

fn bar() -> ! {
}
