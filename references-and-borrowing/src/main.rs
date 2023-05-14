fn main() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("Hello");
    change(&mut s2);
    println!("{}", s2);

    let mut s = String::from("Hello");
    let b1 = &mut s;
    println!("{}", b1);
    let b2 = &mut s; // This second borrow is allowed because the borrowed reference has been used.
    println!("{}", b2);

    {
        let _b1 = &mut s;
    }
    let b2 = &mut s; // This is allowed because its different scopes.
    println!("{}", b2);

    let b1 = &s;
    let b2 = &s;
    let b3 = &s;
    println!("{} {} {}", b1, b2, b3);

    let b1 = &s;
    let b2 = &s;
    println!("{} {}", b1, b2);
    let b3 = &mut s; // This is allowed because b1 and b2 aren't used after the previous println!
    println!("{}", b3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but because it is a reference Rust wont drop s1 because s didn't take ownership. This is called borrowing.

fn change(s: &mut String) {
    s.push_str(", world!");
}
