// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

impl Point<f64, f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct NewPoint<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> NewPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: NewPoint<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 5.0, y: 10.0 };
    let integer_and_float = Point { x: 5, y: 10.0 };
    let _x = integer_and_float.x();
    let _y = integer_and_float.y();
    let float_and_float = Point { x: 5.0, y: 10.0 };
    let distance = float_and_float.distance_from_origin();
    println!("{}", distance);

    let p1 = NewPoint { x: 1, y: 5.0 };
    let p2 = NewPoint { x: "Hello", y: 'w' };
    let p3 = p1.mixup(p2);

    println!("{}, {}", p3.x, p3.y)
}
