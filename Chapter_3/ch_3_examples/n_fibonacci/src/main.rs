use std::io;

fn main() {
        loop{
	    println!("Please input which sequence number to display:");
	
	    let mut guess = String::new();
		let mut fib_num = 0;
        let mut prev_2 = 0;
        let mut prev_1 = 1;

	    io::stdin().read_line(&mut guess)
	    	.expect("Failed to read line");
	
        let guess: u32 = match guess.trim().parse(){
			Ok(num) => num,
			Err(_) => break,
		};

        let temp = guess + 1;
        for i in (1..temp){
            fib_num = prev_2 + prev_1;
            prev_2 = prev_1;
            prev_1 = fib_num;
        }
        
        println!("The {}th term is {}", guess, fib_num);
	}
}
