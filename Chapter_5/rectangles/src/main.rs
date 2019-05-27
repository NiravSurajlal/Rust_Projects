#[derive(Debug)]
struct Rectangle{
    length: u32,
    breadth: u32,
}

impl Rectangle{
    fn area3(&self) -> u32{
    //&Rectangle because want to borrow so main can continue to use rec2
    //rect is immutable borrow type
    self.length * self.breadth
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.breadth > other.breadth && self.length > other.length
    }
}
fn main() {
    //get area by defining length and width, passing to fn to calc
    //OR use tuples:
    let rec1 = (30, 50);
    println!("area is tuple {}", area(rec1));
    //what if we want to know which is width or height

    //use struct

    let rec2 = Rectangle{length: 50, breadth: 30};
    //&Rectangle because want to borrow so main can continue to use rec2
    println!("area is struct {}", area2(&rec2));

        //printing struct
    println!("rec2 is {:#?}", rec2);
    //{}causes an error .. read debug window.. add {:?}
    //still error add #[derive(Debug)] above struct
    //try {:#?} with #[derive(Debug)]

    println!();

/*************METHOD STUFF OUUTPUTS **********/
    //use rec2
    //method println
    println!("area from method {}", rec2.area3()); 

    let rect1 = Rectangle{ breadth: 30, length: 50};
    let rect2 = Rectangle{ breadth: 10, length: 40};
    let rect3 = Rectangle{ breadth: 60, length: 45};

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn area(dimensions: (u32, u32)) -> u32{
    dimensions.0 * dimensions.1
}

fn area2(rect: &Rectangle) -> u32{
    //&Rectangle because want to borrow so main can continue to use rec2
    //rect is immutable borrow type
    rect.length * rect.breadth
}
