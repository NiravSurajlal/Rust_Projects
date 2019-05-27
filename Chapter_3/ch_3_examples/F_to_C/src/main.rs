use std::io;

fn main() {
    //(F-32)*(5/9) = C;
    // let f: f32 = 60.0;
    // let degree_c: f32 = (f - 32.0) * (5.0/9.0);

    // println!("In Celsius: {} from F {}", degree_c, f);
    loop{
	    println!("Please input Farhenhite temp:");
	
	    let mut guess = String::new();
		let mut degree_c: f32 = 32.0;

	    io::stdin().read_line(&mut guess)
	    	.expect("Failed to read line");
	
        let guess: f32 = match guess.trim().parse(){
			Ok(num) => num,
			Err(_) => break,
		};

		degree_c = (guess - 32.0) * (5.0/9.0);
    	println!("{}F is equal to {}C", guess, degree_c);
	}

}
