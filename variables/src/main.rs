fn main() {
    let mut x = 5;
    println!("The value of x is {x} at this point");
    x = 6;
    println!("The value of x is {x} at this point");

    // Declare constant
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of the constant is {}", THREE_HOURS_IN_SECONDS);

    //Outer shadowing
    let y = 10;
    let y = y * 8;

    {
        // Inner shadowing
        let y = y + 500;
        println!("Inner shadowed y has the value of {y}");

        // Shadowing ends here
    }

    println!("The value of y here is {y}");
}
