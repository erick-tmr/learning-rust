fn main() {
    println!("Hello, world!");

    another_function();
    print_labeled_measurement(5, 'h');

    // statements and expressions

    let y = {
        let x = 3;
        x + 1 // line doesn’t have a semicolon at the end, Expressions do not include ending semicolons.
        // if you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value
    };

    println!("The value of y is: {}", y);

    // returning type
    let x = five();

    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function.");
}

// in function signatures, you must declare the type of each parameter
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

// declaring function return type
fn five() -> i32 {
    5
}
