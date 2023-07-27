pub fn type_inference(){

    // Integers
    let my_number = 8;
    let my_number_2: u8 = 8;

    let small_number = 10u8; // 10u8 = 10 of type u8

    let small_number = 10_u8; // This is easier to read
    let big_number = 100_000_000_i32; // 100 million is easy to read with _

    // Floats : 
    // f32, (default) 
    // f64, 


    let my_float = 5.; // Rust sees . and knows that it is a float

    // Of course, only floats of the same type can be used together. 
    // So you can't add an f32 to an f64.

    let my_float: f64 = 5.0; // This is an f64
    let my_other_float: f32 = 8.5; // This is an f32

    let third_float = my_float + my_other_float; // ERROR

    // one solution

    let my_float = 5.0; // This is an f64
    let my_other_float: f32 = 8.5; // This is an f32

    let third_float = my_float + my_other_float; // OK NOW 
     println!("{third_float}");

}