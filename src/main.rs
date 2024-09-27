fn main() {
    // ex1();
    // ex2();
    // ex3()
    ex4()

}

fn ex1(){
    let mut x = 5; //mut for mutable else it would be automatically const
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn ex2() {
    let x = 5;

    let x = x + 1; // 6 duh

    {
        let x = x * 2; // this lives for only as long as the scope
        println!("The value of x in the inner scope is: {x}");
    }
    // x here returns to x + 1 before the scope
    println!("The value of x is: {x}");
}

fn ex3() {
    let spaces = "   ";
    let spaces = spaces.len();

    // ? let mut spaces: &str = "   ";
    // ? spaces: &str = spaces.len(); 
    // ! ERROR THIS RESULTS IN COMPILE ERROR BECAUSE THE TYPE IS THE SAME

    println!("len of space is: {}",spaces)
}

fn ex4(){
    let tup = (500, 6.4, 1); // inferred typed
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("1: {}, 2: {}, 3: {}", five_hundred, six_point_four, one)
}