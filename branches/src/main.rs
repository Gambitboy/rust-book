fn main() {
    let number = 7;

    if number < 5 {
        // arm
        println!("condition was true");
    } else {
        // arm
        println!("condition was false");
    }

    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if is an expression
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("Value of number is: {number}");

    loop {
        // Infinite
        println!("again!");
        break;
        // continue; // skips any remaining code and starts again
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("The value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    convert_fahrenheit_to_celcius();
    nth_fibonacci_number();
}

fn convert_fahrenheit_to_celcius() {
    let farenheit = 1;
    let celcius = (farenheit - 32) * 5 / 9;

    println!("{farenheit} farenheit = {celcius} celcius");
}

fn nth_fibonacci_number() {
    const NTH: u32 = 20;

    let result = fib(NTH);
    println!("The fibonacci number at index {NTH} is: {result}");
}

fn fib(number: u32) -> u32 {
    let condition = number <= 2;

    if condition {
        return 1;
    }

    fib(number - 1) + fib(number - 2)
}
