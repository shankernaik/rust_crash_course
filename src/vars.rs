pub fn run() {
    let name = "Brad";
    let mut age = 37;
    println!("My name is {}, I am {} ", name, age);

    age = 38; // added mut above for this statement

    println!("My name is {}, I am {} ", name, age);

    // Define const
    const ID: i32 = 001;
    println!("Constan ID :{} ", ID);

    //Assign multiple variables
    let (my_name, my_age) = ("Scot", 45);
    println!("My name is {}, I am {} ", my_name, my_age);
}
