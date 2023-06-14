fn main() {
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    //operations();
    //booleans();
    //tuples();
    //array_out_of_bounds_test();
    another_function(5);
    print_labeled_measurements(5, 'h');
}

fn operations() {
    let sum = 5 + 10;
    println!("Sum is: {sum}");

    let difference = 95.5 - 4.3;
    println!("Difference is: {difference}");

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    println!("Quotient is: {quotient} \nTruncated is: {truncated}");

    let remainder = 43 % 5;
    println!("Remained is: {remainder}");
}

fn booleans() {
    let t = true;

    let f: bool = false;
}

fn characters() {
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat: char = '😻';
}

fn tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let five_hundred = tup.0;
    println!("The first value is: {five_hundred}")
}

fn arrays() {
    let a = [1, 2, 3, 4, 5];

    // this is the same as doing let a = [3,3,3,3,3];
    let a = [3; 5];

    let first = a[0];
    let second = a[1];
}

use std::io;

fn array_out_of_bounds_test() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

// Functions

// Rust uses snake case
fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn expressions() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
