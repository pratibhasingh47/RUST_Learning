fn main() {

    // ------------Ownership rules---------------
    // 1. Each value in Rust has a variable that's called its owner
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    {
        // s is not calid here, it's not yet declared
        let _s = "hello";
        // s is valid from this point forward
        // do stuff with s
    } 
    // this scope is now over, and s is no longer valid


    let x = 5;
    let _y = x; //Copy

    let s1 = String::from("hello");
    // let s2 = s1; // Move (not shallow copy)
    let _s2 = s1.clone();

    println!("{} world!",s1 );

    let s3 = String::from("Ownership");
    takes_ownership(s3);
    // println!("{}", s3);

    let x = 47;
    makes_copy(x);
    println!("{}",x);

    let str = gives_ownership();
    println!("str = {}",str);

    let str1 = gives_ownership();
    let str2 = String::from("Hellooo");
    let str3 = takes_give_back(str2);
    println!("s1 = {}, s3 = {}",str1,str3);


    let a = String::from("Heelloo");
    let len = calculate_length(&a);
    println!("The length of '{}' is {}. ", a , len);


    let mut b = String::from("Change");
    change(&mut b);

    // let mut z = String::from("Error");
    // let r1 = &mut z;
    // let r2 = &mut z;
    // println!("{} , {}", r1,r2);

    let z = String::from("Error");
    let r1 = &z;
    let r2 = &z;
    println!("{} , {}", r1,r2);

    // let reference_to_nothing = dangle();

    // Rules of References
    // 1. At any given time, you can have either one mutable reference or any no. of inmutable references.
    // 2. References must always be valid.

    // let mut s = String::from("String Word");
    // let word = first_word(&s);
    // let first = &s[0..6];
    // let second = &s[7..12];
    // s.clear();

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let _word = first_word(&my_string[0..6]);
    let _word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let _word = first_word(&my_string_literal[0..6]);
    let _word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word(my_string_literal);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);




}

fn takes_ownership(some_string: String){
    println!("{}",some_string);
}

fn makes_copy(some_integer:i32){
    println!("{}",some_integer);
}

fn gives_ownership() ->String{
    let some_string = String::from("Hello");
    some_string
}

fn takes_give_back(a_string: String) -> String{
    a_string
}

fn calculate_length(s : &String) -> usize{
    let length = s.len();
    length
}

fn change(some_string: &mut String){
    some_string.push_str(", World");     
}

// fn dangle() -> &String{
//     let s = String::from("Hellllooo");

//     &s
// }

// fn first_word(s: &String ) -> usize{
//     let bytes = s.as_bytes();

//     for(i,&item) in bytes.iter().enumerate(){
//         if item == b' '{
//             return i;
//         }
//     }

//     s.len()
// }


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
