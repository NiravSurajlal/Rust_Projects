use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");
	
	let secret_number = rand::thread_rng().gen_range(1,101);
	
	loop{
	    println!("Please input your guess.");
	
	    let mut guess = String::new();

	    io::stdin().read_line(&mut guess)
	    	.expect("Failed to read line");
	
        let guess: i32 = match guess.trim().parse(){
			Ok(num) => num,
			Err(_) => continue,
		};

        let checked = Guess::new(guess);
		
    	println!("You guessed: {:?}", checked);

	    match guess.cmp(&secret_number){
	        Ordering::Less => println!("Too small!"),
		    Ordering::Greater => println!("Too large!"),
		    Ordering::Equal => {
			    println!("You win!");
				break;
			}
            
	    }
	}
}

#[derive(Debug)]
pub struct Guess{
    // private
    value: i32,
}
impl Guess{
    pub fn new(value: i32) -> Guess{
        if value < 1 || value > 100 {
            println!("Secret number is between 1 & 100, given {}", value);
        }

        // if value passes test, create new guess with its value field set to value parameter and return Guess
        Guess {
            value
        }
    }
    // used to return private value. Can only use with Guess::new so must go through check
    pub fn value(&self) -> i32{
        self.value
    }
}
