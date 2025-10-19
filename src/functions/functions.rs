//Simple function definition and call
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}

// Function with parameters
fn main_with_params() {
    let x = 5;
    print_value(x);
}

fn print_value(value: i32) {
    println!("The value is: {}", value);
}

// Function with return value
fn main_with_return() {
    let x = 5;
    let result = double_value(x);
    println!("The double of {} is {}", x, result);
}

fn double_value(value: i32) -> i32 {
    value * 2
}

fn main_fn_signatures() {
    another_fn(20, 30);
   
}
 
fn another_fn(x: i32, y: i32) {
    println!("Another function with params: {}, {}", x, y);
}

//Statements and Expressions in Function Bodies
fn main_fn_body() {
    
    let x = 5;
    // This is a statement
    let y = {
        // This is an expression
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
  
}