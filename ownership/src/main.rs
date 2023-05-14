fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("{}", s2); // s1 is moved to s2 because its uses heap to store Hellow

    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y); // Because it is stored on the stack, so x doesn't get moved

    takes_ownership(s1);
    makes_copy(x);
    let s3 = takes_and_gives_back(s2);

    println!("{}", s3);
    let (s2, len) = calculate_length(s3);
    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // Because s1 was copied to some_string, at the end of this function drop is called and the memory is free, it will not be given back to s1

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
} // Returns transfer ownership

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
