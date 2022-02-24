
pub(crate) fn if_control() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

pub(crate) fn multiple_if_control() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

pub(crate) fn loop_control() {
    let mut counter = 0;
    loop {
        if counter == 10{
            break
        }
        println!("again! {}", counter);
        counter+=1;
    }
}

pub(crate) fn returning_value_fromm_a_loop() {
    let mut counter = 0;
    let result = loop {
        if counter == 10 {
            break counter * 2;
        }
        counter += 1;
    };
    println!("The result is {}", result);
}

pub(crate) fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

pub(crate) fn looping_through_an_array(){
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}

pub(crate) fn looping_through_an_array_using_a_for_loop() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }
}

pub(crate) fn looping_using_range() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    let x = 5;
    for number in 0..x {
        println!("{}!", number);
    }
}