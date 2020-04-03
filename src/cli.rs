use std::env;
pub fn run() {
    /*
     run the code as :
     $ cargo run Paul
        Finished dev [unoptimized + debuginfo] target(s) in 0.00s
            Running `target/debug/rust_crash_course Paul`
    Args : ["target/debug/rust_crash_course", "Paul"]
    command : Paul
    $ cargo run hello
        Finished dev [unoptimized + debuginfo] target(s) in 0.00s
            Running `target/debug/rust_crash_course hello`
    Args : ["target/debug/rust_crash_course", "hello"]
    command : hello
    Hi , Brad
    */
    let args: Vec<String> = env::args().collect();

    let command = args[1].clone();
    let name = "Brad";
    println!("Args : {:?}", args);

    println!("command : {}", command);

    if command == "hello" {
        println!(" Hi , {}", name);
    }
}
