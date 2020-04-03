// arrays - fixed list of same data types element

use std::mem;
pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);
    println!(" Single value : {}", numbers[0]);

    numbers[2] = 30; // added mut above

    println!("Array : {:?}", numbers);

    println!(" Array length : {}", numbers.len());

    //Array are stack allocated
    println!("Array occupies {} bytes ", mem::size_of_val(&numbers));

    // get slice from array
    let slice: &[i32] = &numbers; // &numbers[0..2] -> first two elements in slice
    println!("Slice : {:?}", slice);
}
