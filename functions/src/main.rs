fn main() {
    //another_function((5, 6));
    let x = plus_one(five());
    println!("the value of x is: {}",x)
}

fn another_function(x: (i32,i32)) {
    println!("The value of x is: {}", x.0);
    println!("The value of y is: {}", x.1);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
