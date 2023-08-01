fn main() {
    // Shadowing

    let my_variable = 8;
    println!("{my_variable}");

    let my_variable = String::from("Hello");
    println!("{}", my_variable);

    println!("-----------------");

    let quantity = 8; // This is an i32
    println!("{}", quantity); // prints 8
    let quantity = 9.2; // This is an f64 with the same name. But it's not the first quantity - it is completely different!
    println!("{}", quantity); // Prints 9.2

    println!("-----------------");

    let my_number = 8; // This is an i32
    println!("{}", my_number); // prints 8
    {
        println!("{}", my_number); // prints 8

        let my_number = 9.2; // This is an f64. It is not my_number - it is completely different!
        println!("{}", my_number) // Prints 9.2
                                  // But the shadowed my_number only lives until here.
                                  // The first my_number is still alive!
    }
    println!("{}", my_number); // prints 8

    println!("-----------------"); // advantage of shadowing

    let final_number = {
        let y = 10;
        let x = 9; // x starts at 9
        let x = times_two(x); // shadow with new x: 18
        let x = x + y; // shadow with new x: 28
        x // return x: final_number is now the value of x
    };
    println!("The number is now: {}", final_number);

    println!("-----------------"); // without shadowing

    let final_number = {
        let y = 10;
        let x = 9; // x starts at 9
        let x_twice = times_two(x); // second name for x
        let x_twice_and_y = x_twice + y; // third name for x!
        x_twice_and_y // too bad we didn't have shadowing - we could have just used x
    };
    println!("The number is now: {}", final_number);
}

fn times_two(number: i32) -> i32 {
    number * 2
}
