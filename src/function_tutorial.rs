
pub(crate) fn my_println(){
  println!("Testing my_println");
    another_function(2);
    print_labeled_measurement(2,'h');
    expression_function();
    let returnedValue = functionThatReturnsAValue();
    println!(">>my_println<< The returned value of  is: {}", returnedValue);
    let returnedValue = function_that_returns_a_value();
    println!(">>my_println<< The returned value of  is: {}", returnedValue);

}

fn another_function(x: i32) {
    println!(">>another_function<< The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!(">>another_function<< The measurement is: {}{}", value, unit_label);
}

fn expression_function() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn functionThatReturnsAValue()-> i32{
    2
}

fn function_that_returns_a_value()-> i32{
    return 10;
}

/***
Functions with Return Values
Functions can return values to the code that calls them.
We don’t name return values, but we do declare their type after an arrow (->).
In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function.
You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly.
Here’s an example of a function that returns a value:
*/