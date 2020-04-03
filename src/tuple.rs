// tuple has max 12 elements, grp of values of different types.

pub fn run() {
    let person: (&str, &str, i8) = ("Brad", "Pitt", 50);

    println!("{}, {}, {}", person.0, person.1, person.2);
}
