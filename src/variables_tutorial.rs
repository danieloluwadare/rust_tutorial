
pub(crate) fn test_variable(){
    println!("Hello, world!");
    let x = 4;
    println!("The value of x is: {}", x);

    // This is not possible due to x is an immutable string
    // x = 6;
    // println!("The value of x is: {}", x);

    // Enable mutability to make use of variable x
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // CONSTANTS
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of const THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);

    // SHADOWING
    /**
    Shadowing is different from marking a variable as mut, because we’ll get a compile-time error if we accidentally try to reassign to this
    variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable
    after those transformations have been completed.

    The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again,
    we can change the type of the value but reuse the same name. For example, say our program asks a user to show how many spaces
    they want between some text by inputting space characters, and then we want to store that input as a number:


     */
    let m = 5;

    let m = m + 1;

    {
        let m = m * 2;
        println!("The value of m in the inner scope is: {}", m);
    }

    println!("The value of m is: {}", m);

    // Showdowing generally
    // The first spaces variable is a string type and the second spaces variable is a number type.
    // Shadowing thus spares us from having to come up with different names,
    let spaces = "   ";
    println!("This is an empty space {}",spaces);
    let spaces = spaces.len();
    println!("{}",spaces);

    // SHADOWING
}