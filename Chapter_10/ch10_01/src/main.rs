// fn largest_i32(list: &[i32]) -> i32 {
//     // function that takes a i32 reference in var called list
//     // concrete slice of i32 values
//     // returns an i32

//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// fn largest_char(list: &[char]) -> char {
//         let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// fn largest_generic<T>(list: &[T]) -> T{
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

struct Point<T> {
    // will they have the same val?
    x: T,
    y: T,
}

struct Point_two<T, U> {
    // will they have the same val?
    x: T,
    y: U,
}

impl<T> Point<T> {
    // method acting on struct Point
    fn x(&self) -> &T{
        &self.x
    }
}

impl<T, U> Point_two<T, U>{
    fn mixup<V,W>(self, other: Point_two<V, W>) -> Point_two<T,W>{
        Point_two{
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    // let char_list = vec!['a', 'q', 'l', 'b'];

    // /* moved code to determine largest SLICE in list to function*/
    // /* returns largest int or char in SLICE*/ 
    // let result_i32 = largest_i32(&number_list);
    // println!("The largest number is {}", result_i32);

    // let result_char = largest_char(&char_list);
    // println!("The largest char is {}", result_char);

    let integger = Point{x: 5, y: 10};
    let float = Point{x: 1.0, y: 4.0};

    println!("integger.x = {}", integger.x());

    let p1 = Point_two{x: 5, y: 10.4};
    let p2 = Point_two{x: "hello", y: 'c'};
    // calls mixup method on p1 with argument p2
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
