fn fib(x: i32) -> i32{
    if x <= 1 {
        1
    } else {
        fib(x-1) + fib(x-2)
    }
}

fn main() {
    let result = fib(40);

    println!("result was {}", result);
}
