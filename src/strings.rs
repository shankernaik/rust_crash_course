// Primitive str = "Hello"; is a immutable fixed length string in the memory
// String = growable , heap allocated data structure - when you want to modify or own a string data

pub fn run() {
    let str = "hello";

    let mut astr = String::from("HELLO! ");

    println!("{}", str);
    println!("{}", astr);

    // Get length
    println!(" Length : {}", str.len());

    astr.push('S'); // added mut above, this allow a char of unicode

    astr.push_str("amuel"); // push string. added mut above

    println!(" Name : {}", astr);

    //capacity in bytes
    println!(" Capacity in bytes : {}", astr.capacity());

    //check if empty
    println!(" is empty : {}", astr.is_empty());

    //contains a sub string
    println!(" Contains : {}", astr.contains("Sam"));

    //contains a replace
    println!(" Replace : {}", astr.replace("Sam", "Man"));

    //split by whitespaces
    for word in astr.split_whitespace() {
        println!(" split : {}", word);
    }

    //create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // asertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}
