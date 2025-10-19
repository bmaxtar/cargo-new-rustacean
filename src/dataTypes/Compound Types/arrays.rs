fn main () {
    let a = [1, 2, 3, 4, 5];
    //Accessing Array Elements
    let first = a[0];
    let second = a[1];
    let third = a[2];
    let fourth = a[3];
    let fifth = a[4];
    println!("{} {} {} {} {}", first, second, third, fourth, fifth);

    let months_of_the_year = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    //Accessing Array Elements
    let first = months_of_the_year[0];
    let second = months_of_the_year[1];
    let third = months_of_the_year[2];
    let fourth = months_of_the_year[3];
    let fifth = months_of_the_year[4];
    println!("{} {} {} {} {}", first, second, third, fourth, fifth);

    //Invalid Array Element Access
    let x = [1, 2, 3, 4, 5];
    let index = 10;
    let element = x[index];  // panic: index out of bounds: the len is 5 but the index is 10
    println!("The value of element is: {}", element);
}