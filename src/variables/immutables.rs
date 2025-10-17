fn main() {
    // let x = 5;
    // x = 6; // This line will cause a compile-time error because `x` is immutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
