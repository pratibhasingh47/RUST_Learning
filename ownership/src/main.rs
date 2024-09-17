fn main() {

    // ------------Ownership rules---------------
    // 1. Each value in Rust has a variable that's called its owner
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    {
        // s is not calid here, it's not yet declared
        let s = "hello";
        // s is valid from this point forward
        // do stuff with s
    } 
    // this scope is now over, and s is no longer valid


    let x = 5;
    let y = x; //Copy

    let s1 = String::from("hello");
    // let s2 = s1; // Move (not shallow copy)
    let s2 = s1.clone();

    println!("{} world!",s1 );

    let s3 = String::from("Ownership");
    takes_ownership(s3);
    println!("{}", s3);

    let x = 47;
    makes_copy(x);
    println!("{}",x);

    let str = gives_ownership();
    println!("str = {}",str);

    let str1 = gives_ownership();
    let str2 = String::from("Hellooo");
    let str3 = takes_Give_back(str2);
    println!("s1 = {}, s3 = {}",str1,str3);
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

fn takes_Give_back(a_string: String) -> String{
    a_string
}