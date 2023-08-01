fn number() -> i32 {
    8
}

fn number_2() -> i32 {
    return 8;
}

//---------------------------------------------------------

fn multiply(number_one: i32, number_two: i32) {
    // Two i32s will enter the function.
    // We will call them number_one and number_two.
    let result = number_one * number_two;
    println!("{} times {} is {}", number_one, number_two, result);
}

//---------------------------------------------------------

fn multiply_2(number_one: i32, number_two: i32) -> i32 {
    let result = number_one * number_two;
    println!("{} times {} is {}", number_one, number_two, result);
    result // this is the i32 that we return
}

//---------------------------------------------------------

pub fn main() {
    println!("Hello, world!");

    println!("Hello, worlds number {} and {}!", 8, 9);

    println!("Hello, world number {}!", number());
    println!("Hello, world number {}!", number_2());

    println!("{:?}", multiply(2, 7));
    println!("The result of multiply_25(7, 4) = {:?}", multiply_2(7, 4));
}
