#![allow(unused_variables)]
fn main() {
    let mut s;
    {
        s = String::from("hello");
        s.push_str(", world!");
        let s2 = s.clone();
        //s = String::from("test");

        println!("internal scope: {}", s);
        println!("internal scope:s2 {}", s2);

    } 
    println!("external scope: {}", s);

    takes_ownership(s);

    s = give_ownership();

    println!("return fromm give_ownership: {}", s);

    println!("length of string: {}", calc_len(s));
    
}

fn calc_len(str: String) -> usize {
    str.len()
}
fn give_ownership() -> String {
    let s = String::from("Happy code");
    s
}
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

