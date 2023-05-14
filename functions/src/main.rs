fn main() {
    println!("Hello, world!");

    another_function(5, 'h');

    let _y = 6; // Statment - does not return a value on creation
    let y = 6;
    let x = y + 2; // This operation is an expression
    println!("The value of x is: {x}");

    let y = {
        let x = 3;
        x + 1 // NOTES: expressions don't have ; otherwise it turns into a statement
    }; // Scope blocks are expressions
    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn another_function(value: i32, unit_label: char) {
    println!("The value of value is: {value}{unit_label}");
}

fn five() -> i32 {
    5 // No ; so it returns the 5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
