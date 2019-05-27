fn main() {
    let s1 = String::from("ownership given");
    take_ownership(s1);

    let x = 5;
    makes_copy(x);

    // println!("{}", x);      //s1 does not work because moved out in fn "take_ownership"

//**************************************************/
    let s2 = give_ownership();
    println!("{}", s2);

    let s3 = String::from("ownership both");
    let s4 = take_give(s3);
    println!("{}", s4);
    //s's out of scope. 4 dropped, 3 nothing because moved, 2 dropped

//************************************************ */
    let s5 = String::from("Tuple");
    let (s6, len) = calc_len(s5);

    println!("{} with string length {}", s6, len);
}

fn take_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_int: i32){
    println!("{}", some_int);
}

fn give_ownership() -> String{
    //moves return avl into fn that calls it
    let some_string = String::from("ownership acquired");
    some_string //returned ... not semi-colon    
}

fn take_give(a_string: String) -> String{
    a_string
}

fn calc_len(s: String) -> (String, usize){
    let length = s.len();

    (s, length)
}