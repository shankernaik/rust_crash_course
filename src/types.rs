pub fn run() {
    // Rust is static type lang.

    // default type i32 (type inference)
    let x = 1;

    // explicit type
    let y: i64 = 3333333333;

    let z = 2.5;

    println!("{}", std::i32::MAX);
    println!("{}", std::i64::MAX);

    // boolean type
    let is_active = true; // OR let is_active : bool = true;

    let a1 = 'a'; // the character is a unicode, let a1 = 'ab'; won't work

    let face = '\u{1F600}'; //emoji unicode

    println!("{:?}", (x, y, z, is_active, a1, face));
}
