// vector - is a resizable array

use std::mem;
pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // add on vector
    numbers.push(6);
    numbers.pop();
    println!("{:?}", numbers);
    println!(" Single value : {}", numbers[0]);

    numbers[2] = 30; // added mut above

    println!("Vector : {:?}", numbers);

    println!(" Vector length : {}", numbers.len());

    //Vector are stack allocated
    println!("Vector occupies {} bytes ", mem::size_of_val(&numbers));

    // get slice from Vector
    let slice: &[i32] = &numbers[0..2]; // -> first two elements in slice
    println!("Slice : {:?}", slice);

    // loop through vector
    for x in numbers.iter() {
        println!("Number x : {} ", x);
    }

    //loop an mutate value
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Number vector : {:?} ", numbers);
}
