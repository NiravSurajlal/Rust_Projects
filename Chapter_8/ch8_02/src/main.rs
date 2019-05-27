fn main() {
    //different ways to create strings
    {
        // let mut s1 = String::new();
        let data = "initial contents 1";
        let s1 = data.to_string();
        println!("{}", s1);

        // let mut s2 = String::new();
        let s2 = "initial contents 2".to_string();
        println!("{}", s2);

        let s3 = String::from("initial contents 3");
        println!("{}", s3);
    }

    //updating a string
    {
        let mut s4  = String::from("first");
        s4.push_str(" second");
        println!("{}", s4);
    }

    //concatenate strings
    {
        let c1 = String::from("Part1, ");
        let c2 = String::from("Part2");
        let c3 = c1 + &c2;
        println!("{}", c3);
        // println!("{}", c1) will not work because ownership given

        let c4 = String::from("Part1");
        let c5 = format!("{} & {}", c4, c2);
        println!("{}", c5);
    }

    // string slicing and "indicies"
    {
        let hello = "Здравствуйте";
        let s = &hello[0..4];
        println!("{} ... {}", hello, s);
    }

    // iterating through strings
    {   
        println!("नमस्ते");
        for c in "नमस्ते".chars() {
            print!("{}", c);
        }
    }
}
