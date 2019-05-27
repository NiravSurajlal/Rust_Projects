fn main() {
    // infer type, or can use let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3, 4, 5];
    // third is a pointer type that points to the location in memory where v[2] is stored
    let third: &i32 = &v[2];
    println!("Third element is {}", third);

    match v.get(2){
        Some(third) => println!("Third element is {}", third),
        None => println!("No third element"),
    }

    let v2 = vec![1, 2, 3, 4, 5];;
        // causes error
    // let fail = &v[100];
        // returns none
    let fail = v.get(100);

    enum Spreadsheet{
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        Spreadsheet::Int(3),
        Spreadsheet::Text(String::from("blue")),
        Spreadsheet::Float(10.12),
    ];
}
