fn main() {
    let tuple: (i32, f64, u8) = (30, 12.8, 6);
    // Pattern matching
    let (x, y, z) = tuple;
    println!("Tuple members are: {x}, {y}, and {z}");
    // Access by index
    println!("X: {}, Y: {}, Z: {}", tuple.0, tuple.1, tuple.2);
    // Array
    let a: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    // Array initialized
    let a_initialized = [7; 12];

    let first_of_a = a[0];
    let first_of_a_initialized = a_initialized[0];

    println!("A_0: {first_of_a}, A_INITIALIZED_0: {first_of_a_initialized}");
}
