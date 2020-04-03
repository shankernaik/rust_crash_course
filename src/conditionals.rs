// arrays - fixed list of same data types element

pub fn run() {
    let age: u8 = 18;
    let check_id: bool = true;

    // if/else
    if age >= 21 {
        println!(" What do you want? ");
    } else if age < 21 && check_id {
        println!(" Leave now");
    } else {
        println!(" show id ");
    }

    //shorthand if
    let is_of_age = if age >= 21 { true } else { false };

    println!(" Is drinking age , {}", is_of_age);
}
