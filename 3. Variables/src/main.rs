fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const SUBSCRIBER_COUNT: u32 = 100_000;


    // datatypes

    //scalar datatypes

    //integer
    //floating point numbers
    //boolean
    //character

    //integer 
    let a = 98_222; // decimal
    let b = 0xff; // hex
    let c = 0o77; // octal
    let d = 0b1111_0000; // binary
    let e = b'A'; // byte (u8 only)

    let f: u8 = 255;

    //floating point numbers

    let f = 2.0;
    let g: f32 = 3.0;

    //addition
    let sum = 5 + 10;
    //subtraction
    let difference = 95.5 - 4.3;
    //multiplication
    let product = 4 * 30;
    //division
    let quotient = 56.7 / 32.2;
    //remainder
    let remainder = 43 % 5;

    //boolean
    let t = true;
    let f: bool = false;

    //character
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    //compound types
    let tup: (&str, i32) = ("Let's get Rusty!", 100_000);
    let (channel, sub_count) = tup;
    let sub_count = tup.1;

    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];

    let byte = [0; 8];

    let sum = my_function(11, 22);
    println!("The value of sum is: {}", sum);

    //control flow
    let number = 5;
    
    if number < 10 {
        println!("first condition was true");
    } else if number < 22 {
        println!("second condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    let mut counter = 0;

    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    // Line comment

    /*
    Block comment
    */

}

//functions
fn my_function(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    x + y //return value
}