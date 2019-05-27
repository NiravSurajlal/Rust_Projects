fn main() {
    println!("Hello, world!");

    let x = Coin::Quarter(UsState::Alaska);
    let m = value_in_cents(x);
    println!("{}", m); //match assigns value

    //using match and Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    //println!("{:?} {:?}", six, none);

    //placeholder? only works for integers as patterns?
    let some_u8 = 0u8;
    match some_u8{
        1 => println!("one"),
        _ => (),
    }

    //using if let
    let a_u8_val = Some(0u8);
        // match a_u8_val{
        //     Some(3) => println!("Three"),
        //     _ => (),
        // }
    if let Some(3) = a_u8_val{
        println!("Three");
    }
}

#[derive(Debug)] 
enum UsState{	
    Alabama, 
    Alaska, 
    Arizona, 
    Arkansas, 
    //rest of states
}
enum Coin{
    Penny,
    Nickel, 
    Dime,
    Quarter(UsState),
    //more things to test placeholder
    // Dollar,
    // Pound,
    // Rand, 
    // Rupee,
}
fn value_in_cents(coin: Coin) -> u32{
    match coin{
        Coin::Penny => {
            println!("Lucky Penny");
            1},
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {println!("State quarter from {:?}", state);
            25},
    }
}

fn plus_one(x: Option<i32>)->Option<i32>{
    match x{
        None => None,
        Some(i) => Some(i+1),
    }
}

