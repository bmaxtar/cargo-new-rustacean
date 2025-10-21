fn main() {
    // --- Simple function definition and call ---
    println!("Hello, world!");
    print_another_function();
    println!(); // for spacing

    // --- Function with parameters ---
    let x_params = 5;
    print_value(x_params);
    println!();

    // --- Function with a return value ---
    let x_return = 5;
    let result = double_value(x_return);
    println!("The double of {} is {}", x_return, result);
    println!();

    // --- Another example of function parameters ---
    print_two_params(20, 30);
    println!();

    // --- Statements and Expressions in Function Bodies ---
    // A statement performs an action but does not return a value. `let y = ...;` is a statement.
    let y = {
        // This block is an expression. Its value is the value of the last line.
        let x_expr = 3;
        x_expr + 1 // No semicolon means this is an expression, and its value is returned.
    };
    println!("The value of y (from an expression block) is: {}", y);
    println!();

    // --- Functions with implicit return values ---
    let x_five = five();
    println!("The value of x (from five()) is: {}", x_five);
}

fn print_another_function() {
    println!("Another function.");
}

fn print_value(value: i32) {
    println!("The value is: {}", value);
}

fn double_value(value: i32) -> i32 {
    value * 2 // This is an expression, its value is returned.
}

fn print_two_params(x: i32, y: i32) {
    println!("Function with two params: {}, {}", x, y);
}

fn five() -> i32 {
    5 // The value 5 is returned implicitly.
}
