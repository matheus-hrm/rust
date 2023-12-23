fn main() {
    println!("{}", fib(4))
}

fn fib(x:i32) -> i32 {
    if x == 0 {
        return 0;
    } else if x == 1 {
        return 1;
    } else {
        return fib(x-1) + fib(x-2);
    } 
}