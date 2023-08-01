fn main() {
    let my_number = 10; // let binding

    println!("my number is : {my_number}");

    //-----------------------------------

    let my_number = {
        let second_number = 8;
        second_number + 9 // No semicolon, so the code block returns 8 + 9.
                          // It works just like a function
    };

    println!("My number is: {}", my_number);

    //-----------------------------------

    let my_number = {
        let second_number = 8; // declare second_number,
        second_number + 9; // add 9 to second_number
                           // but we didn't return it!
                           // second_number dies now
    };

    println!("My number is: {:?}", my_number); // my_number is ()
}
