fn largest_i32(list: &[i32]) -> i32 {
    // function that takes a i32 reference in var called list
    // concrete slice of i32 values
    // returns an i32

    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_generic<T>(list: &[T]) -> T{
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['a', 'q', 'l', 'b'];

    /* moved code to determine largest SLICE in list to function*/
    /* returns largest int or char in SLICE*/ 
    let result_i32 = largest_i32(&number_list);
    println!("The largest number is {}", result_i32);

    let result_char = largest_char(&char_list);
    println!("The largest char is {}", result_char);

}
