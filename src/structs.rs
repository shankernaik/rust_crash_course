// normal struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// tuple struct
struct Tcolor(u8, u8, u8);

// person struct
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}
pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    println!("Color : {} {} {}", c.red, c.green, c.blue);
    c.red = 200;
    println!("Color : {} {} {}", c.red, c.green, c.blue);

    let mut tc = Tcolor(255, 0, 0);
    println!("T-Color : {} {} {}", tc.0, tc.1, tc.2);
    tc.0 = 200;
    println!("T-Color : {} {} {}", tc.0, tc.1, tc.2);

    let mut p = Person::new("John", "Doe");
    println!(" Full name : {}", p.full_name());
    p.set_last_name("Hopkins");
    println!(" Full name : {}", p.full_name());
    println!("Person {} {}", p.first_name, p.last_name);
    println!(" Person Tuple : {:?} ", p.to_tuple());
}
