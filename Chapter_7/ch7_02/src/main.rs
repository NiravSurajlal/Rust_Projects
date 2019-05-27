// use std::fmt;
// use std::io;

mod sound{
    pub mod instrument{
        //pub allows access to parent module
        //if we have access to sound, then we have access to instrument
        pub fn clarinet(){
            //similarily, pub grants access to clarinet           
        }
    }
}


mod plant {
    pub struct Vegetable {
        //specific fields declared public
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        //this function allows us to use the struct in main
        //with only one field being public in main
        //the instance of Vegetable created in main comes with id set here, where id can be accessed
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

mod menu{
    pub enum Appetizer{
        Soup,
        Salad,
    }
}
fn main() {
    // absolute path
    //because main fn and sound module defined in same module, 
        //main can refer to sound
    crate::sound::instrument::clarinet();
    // relative path
    //do not decalre crate root
    //main and sound in same module
    sound::instrument::clarinet();

//_______________________________________________________________
    //public structs ->
    let mut v = plant::Vegetable::new("squash");
    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);

    // The next line won't compile if we uncomment it:
    // because id field is private
    // println!("The ID is {}", v.id);

//_______________________________________________________________
    //public enums ->
    //can use Soup and Salad variants in main
    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;
}

//_______________________________________________________________
//both have Result as parents -> error
// fn function1() -> fmt::Result{
// }