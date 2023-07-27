pub fn primitive_type(){

    
    /*
    
    integers :
    --------- 

    i8, i16, i32, i64, i128, isize
    u8, u16, u32, u64, u128, usize

    usize => 64 bits  * if possible * if not 32 bits
    */

    /*character:
    -------------*/
    let first_letter = 'A';
    let space = ' '; // A space inside ' ' is also a char
    let other_language_char = 'Ꮔ'; // Thanks to Unicode, other languages like Cherokee display just fine too
    let cat_face = '😺'; // Emojis are chars too

    /* casting
    ---------- */

    /* 
    -10 as an i8 is 11110110
    -10 as an i8 is 11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111110110
    
    
     */

    let my_number = 100; // We didn't write a type of integer,
                         // so Rust chooses i32. Rust always
                         // chooses i32 for integers if you don't
                         // tell it to use a different type


    println!("{}", my_number as char); // ⚠️ ERROR

    println!("{}", my_number as u8 as char); // OK --> 'd'

    // Better way: specify the type initially

    et my_number: u8 = 100; //  change my_number to my_number: u8
    println!("{}", my_number as char);

    //-------------------------------

    println!("Size of a char: {}", std::mem::size_of::<char>()); // 4 bytes
    println!("Size of string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
    println!("Size of string containing 'ß': {}", "ß".len());
    println!("Size of string containing '国': {}", "国".len());
    println!("Size of string containing '𓅱': {}", "𓅱".len());

    // rust .len() gibe the size in terms of number of bytes

    
    let slice = "Hello!";
    println!("Slice is {} bytes.", slice.len());
    let slice2 = "안녕!"; // Korean for "hi"
    println!("Slice2 is {} bytes.", slice2.len());

    // If .len() gives the size in bytes, what about the size in characters?

    let slice = "Hello!";
    println!("Slice is {} bytes and also {} characters.", slice.len(), slice.chars().count());
    let slice2 = "안녕!";
    println!("Slice2 is {} bytes but only {} characters.", slice2.len(), slice2.chars().count());
}




}