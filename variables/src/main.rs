fn main() {
    let mut x = 5;
    println!("The calue of x is : {}", x);
    x = 6;
    println!("The value of x is : {}", x);

    let x = 47;
    println!("The calue of x is : {}", x);
    let x = "six";
    println!("The value of x is : {}", x);

    const COUNT: u32 = 10000;
    println!("{}", COUNT);

    // Integers
    let a = 98_222; //Decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let _e = b"A"; // Byte(u8 only)
    let f: u8 = 255;

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
    // println!("{}",e);
    println!("{}", f);
    println!("{} , {} , {}", a, b, f);

    // Floating-point Numbers
    let f = 2.0;
    let g: f32 = 3.0;
    println!("{}", f);
    println!("{}", g);

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
    println!("{}", sum);
    println!("{}", difference);
    println!("{}", product);
    println!("{}", quotient);
    println!("{}", remainder);

    //Booleans
    let t = true;
    let f: bool = false;
    println!("{}", t);
    println!("{}", f);

    //Characters
    let k = 'z';
    let l = 'Z';
    let heart_eyed_cat = '😻';
    println!("{}", k);
    println!("{}", l);
    println!("{}", heart_eyed_cat);

    //Compound Types
    let tup = ("Let's Get Rusty!", 100_000);
    // println!("{}",tup);

    let (channel, _sub_count) = tup;
    let sub_count = tup.1;
    println!("{}", sub_count);
    println!("{}", channel);

    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];
    println!("{}", not_found);

    let _byte = [0; 8]; //crate an array of 8 elements all value to 0

    //FUNCTIONS
    fn my_function(x: i32, y: i32) -> i32 {
        println!("Another function");
        println!("The value of x is: {}", x);
        println!("The value of y is: {}", y);
        let sum = x + y;
        // return sum;
        sum
    }

    let add = my_function(11, 47);
    println!("The addition is {}", add);

    //Control Flow
    let number = 5;
    if number < 10 {
        println!("Less then");
    } else if number > 10 {
        println!("Greater then");
    } else {
        println!("no identified");
    }

    let condition = true;
    let _number = if condition { 5 } else { 6 };

    let mut counter = 0;
    let result = loop {
        println!("Looppp...");
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("The result is {}", result);

    let mut no1 = 5;

    while no1 != 0 {
        println!("{}", no1);
        no1 -= 1;
    }
    println!("While loop breaks");

    let collec = [10, 20, 30, 40, 50, 60, 70];

    for element in collec.iter() {
        println!("The value is {}", element);
    }

    for number in 1..4 {
        println!("{}", number);
    }

    println!("{:?}",collec);
    println!("{:#?}",collec);
}
