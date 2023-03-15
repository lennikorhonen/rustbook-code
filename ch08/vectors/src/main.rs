fn main() {
    // let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3]; // Can use the vec! macro insted of Vec::new()

    println!("Vector is {:?}", v);

    let mut v2 = Vec::new();

    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    println!("Vector 2 is {:?}", v2);

    let v3 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v3[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v3.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    let v4 = vec![100, 32, 57];
    for i in &v4 {
        println!("{i}");
    }

    let mut v4mut = vec![100, 32, 57];
    for i in &mut v4mut {
        *i += 50;
    }

    println!("Mutable vector 4: {:?}", v4mut);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("Spreadsheet row: {:?}", row);
}

