/**
Scalar Types
A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
You may recognize these from other programming languages. Let’s jump into how they work in Rust.
*/
pub(crate) fn integer_data_types(){
     let a:u8 = 1;
     println!("x is an integer of type u8 {}",a);

     let b:u32 = 1;
     println!("b is an integer of type u32 {}",b);

     let c:u64 = 1;
     println!("c is an integer of type u64 {}",c);
 }

pub(crate) fn float_data_types(){
    let a:f64 = 1.0;
    println!("a {} is a float of type f64",a);

    let b=-3.0;
    println!("{}",b)
}

pub(crate) fn numeric_computation() {
    // addition
    let sum = 5 + 10;
    println!("sum of  5 + 10 ==> {} ",sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference of 95.5 - 4.3 ==> {} ",difference);

    // multiplication
    let product = 4 * 30;
    println!("product of 4 * 30 ==> {} ",product);

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient of 56.7 / 32.2; ==> {} ",quotient);

    let floored = 2 / 3; // Results in 0
    println!("floored of  2 / 3 ==> {} ",floored);

    // remainder
    let remainder = 43 % 5;
    println!("remainder of  43 % 5 ==> {} ",remainder);

}

pub(crate) fn boolean_data_type() {
    let t = true;
    println!("t {} is of type boolean ",t);
    let m : bool = false;
    println!("m {} is of type boolean ",m);

    // This wont work it has to be initialized
    // let x : bool;
    // println!("x {} is of type boolean and it defaults",x);


}

pub(crate) fn char_data_type() {
    let t = 'x';
    println!(">>char_data_type<< t {} is of type char ",t);
    let heart_eyed_cat = '😻';
    println!(">>char_data_type<< heart_eyed_cat {} is of type char ",heart_eyed_cat);
}

/**
Compound Types
Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.
 */

/**
    A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    Tuples have a fixed length: once declared, they cannot grow or shrink in size.
 */
pub(crate) fn tuple_data_type() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!(">>tuple_data_type<< The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    println!(">>tuple_data_type<< The value of five_hundred is: {}", five_hundred);

    let six_point_four = x.1;
    println!(">>tuple_data_type<< The value of six_point_four is: {}", six_point_four);

    let one = x.2;
    println!(">>tuple_data_type<< The value of one is: {}", one);

}

pub(crate) fn array_data_type() {
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    /**
        You write an array’s type using square brackets with the type of each element, a semicolon,
        and then the number of elements in the array, like so:
    */
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let first = a[0];
    println!(">>array_data_type<< The value of first is: {}", first);

    let second = a[1];
    println!(">>array_data_type<< The value of second is: {}", second);

}