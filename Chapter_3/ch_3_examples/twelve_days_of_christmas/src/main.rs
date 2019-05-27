fn main() {
    let lyrics = ["A Partridge in a Pear Tree", "Two Turtle Doves", "Three French Hens", 
        "Four Calling Birds", "Five Golden Rings", "Six Geese a Laying", "Seven Swans a Swimming", 
        "Eight Maids a Milking", "Nine Ladies Dancing", "Ten Lords a Leaping", 
        "Eleven  Pipers Piping", "Twelve Drummers Drumming"];
    
    let number = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "nineth", "tenth", "eleventh", "twelveth"];

        for i in (1..13){
            println!("On the {} day of christmas my true love said to me:", number[i-1]);
            for ii in (0..i).rev(){
                if ii != 0 {
                    println!("{}", lyrics[ii]);
                }
                else if i == 1{
                    println!("{}", lyrics[ii]);
                }
                else{
                    println!{"and {}", lyrics[ii]}
                }
            }
        }
}
