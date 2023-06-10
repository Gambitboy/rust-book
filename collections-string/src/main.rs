fn main() {
    let data = "initial contents";

    let _s = data.to_string();
    let _s = "initial contents".to_string();
    let _s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    let _s3 = format!("{s3}!");

    let hello = "Здравствуйте";
    let answer = &hello[0..2];
    println!("{answer}");

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}
