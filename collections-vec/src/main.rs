use std::vec;

fn main() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let vv = vec![1, 2, 3, 4, 5];

    let third: &i32 = &vv[2];
    println!("The third element is {third}");

    let third: Option<&i32> = vv.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is not third element."),
    }

    // let _does_not_exist = &vv[100];
    let _does_not_exist = v.get(100);

    let first = &v[0];

    println!("The first element is {first}");
    v.push(6);

    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    {
        let v = vec![1, 2, 3, 4];
    }
    println!("{:?}", v);
}
