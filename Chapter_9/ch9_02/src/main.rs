use std::fs::File;

fn main() {
    // to ask type? Know not f32 so
    let f: u32 = File::open("hello.txt");
    // let f = File::open("hello.txt");

    let f = match f {
        // when file is okay, return inner file value of the ok variant & assign that file value to f 
        Ok(file) => file,
        // if there is no file (above) 
        Err(error) => {panic!("Problem opening file", error),}
    };
}
