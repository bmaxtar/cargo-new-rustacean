fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    let x = x + 1; // Shadowing the previous `x`
    println!("The value of x after shadowing is: {}", x);

    {
        let x = x * 2; // Shadowing `x` again in a new scope
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x in the outer scope is: {}", x);
}
