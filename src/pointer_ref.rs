pub fn run() {
    //primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!(" arr1 {:?}, arr2 {:?}", arr1, arr2);

    //vector
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1; // use `&` as it is borrowed to vec2

    println!(" vec1 {:?}  ", (&vec1, vec2));
}
