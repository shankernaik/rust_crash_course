pub fn run() {

    //Any function ending with ! is a macro.
    
    // print to console
    println!("Hello from print rs file");

    // Basic formatting
    println!(" Hello , {}", "Sam");

    // positional agruments
    println!(" Hello, {1}, replied {0} ", "Pete", "Sam");

    // named arguments
    println!("{name} likes to play soccer", name = "John");

    // placeholder traits
    println!(" Binary :{:b} , Hex:{:x} , Octal:{:o}", 10, 10, 10);

    // placeholder for debug traits
    println!(" {:?}", (10, "hello", true));

    // basic math
    println!(" {}", 10 + 10);
}
