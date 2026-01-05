fn main() {
    println!("Hello, world!");

    here_is_one_function();
    here_is_another_function();
    here_is_a_parameterized_function(4, 'y');

    let value: i32 = here_is_a_function_that_returns_a_value(-3);
    println!("Value: {value}");
}

fn here_is_one_function() {
    println!("Here is one function");
}

fn here_is_another_function() {
    println!("Here is another function");
}

fn here_is_a_parameterized_function(a: i32, b: char) {
    println!("{a}{b} was entered");
}

fn here_is_a_function_that_returns_a_value(x: i32) -> i32 {
    7 + x
}
