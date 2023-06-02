use crate::garden::vegetables::Asparagus;

pub mod garden;

// Order of finding a module:
//
// inline
// ./module-name.rs
// ./module-name/md.rs

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}", plant);
}
