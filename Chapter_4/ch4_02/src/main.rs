fn main() {
    let mut s1 = String::from("ref to obj");

    let len = calc_len(&s1);    //reference s1 with &

    println!("len of {} is {}", s1, len);

    change(&mut s1); //create mutable reference
    //println!("changed string is", s1);
}

fn calc_len(s: &String) -> usize{
    s.len()
}

fn change(some_string: &mut String){ //accept mutable ref
    some_string.push_str("changed");
}