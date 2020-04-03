pub fn run() {
    greeting("Hello", "Jane");

    //binding function values to variable
    let sum = add(4, 5);
    println!("Sum : {}", sum);

    //closures = allows you to use outside variables which you can't do in normal function due to blocked scope.
    // it uses pipe character
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("c sum = {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you.", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2 // without semicolon  or return n1+n2;
}
