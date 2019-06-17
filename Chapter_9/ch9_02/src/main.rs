use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // to ask type? Know not f32 so
    // let f: u32 = File::open("hello.txt");
    let f = File::open("hello.txt");

    let f = match f {
        // when file is okay, return inner file value of the ok variant & assign that file value to f 
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => {panic!("Tried to create file, but problem occured: {:?}", e)},
            },
            other_error => {panic!("Probelm opening file {:?}", other_error)},
        },

        // if there is no file (above) 
        // Err(error) => {panic!("Problem opening file {:?}", error)},
    };

    /*****Explains above here*****/
    // File::open returns an io::Error inside the Err variant from the Result type it returns, which is a struct in the standard library
    // This struct has a method Kind which can be called to get an io::ErrorKind value 
    //     (enum in the std lib with variants representing different errors that may result from an io operation)
    // Here the variant used is the ErrorKind::NotFound]
    // Thus we match on f, and have an inner match on error.kind()
    // checks what variant the error.kind enum returns, if it is NotFound then ...
    // can fail to create, hence inner error handling
    


}
