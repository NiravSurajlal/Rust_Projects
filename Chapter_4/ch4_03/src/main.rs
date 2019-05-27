fn main() {
    let mut s = String::from("string slices");

    let word = first_word_two(&s[..]);
       //s.clear(); //error caused because cannot take immutable and muttable ref at the same time. Mut ref required here
    println!("{}", word);

    let s_literal = "literal slices";
    let word = first_word_two(&s_literal[..]);
    println!("{}", word);
    //str literls = slices, this works
    let word = first_word_two(&s_literal);
    println!("{}", word);
}

// fn first_word_one(s: String) -> usize{
//     let bytes = s.as_bytes(); //convert str to array of bytes
//     for (i, &item) in bytes.iter().enumerate(){
//         //iter returns each element in collection
//         //enumerate wraps result of iter and returns each element as part of tuple
//         //first element = index, second = reference to element
        
//         //i as index and &item for byte in tuple
//         if item == b' '{
//             //byte literal syntax
//             return i;
//         }
//         //function would break when return
//         s.len();
//     }
// }

fn first_word_two(s: &str) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}