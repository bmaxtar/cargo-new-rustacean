fn main() {
    let guess = "42".parse().expect("Not a number!");
    println!("The guessed number is: {}", guess);
}