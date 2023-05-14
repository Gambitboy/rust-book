fn main() {
    let s = String::from("Hello world");
    let hello = &s[..5];
    let world = &s[6..];
    println!("{}, {}", hello, world);

    let first = first_word(&s);
    println!("{}", first);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3])
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
