// Normal Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
struct Color2(u8, u8, u8);

// With Methods
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

    fn full_name(&self) -> String {
        return format!("{}, {}", self.last_name, self.first_name);
    }

    fn set_title(&mut self, title: &str) {
        self.first_name = format!("{} {}", title, self.first_name);
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

// -----------------------------------------------------------------------------
pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    c.green = 128;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    // ------------------------------------------------

    let c2 = Color2(255, 0, 0);
    println!("Color: {} {} {}", c2.0, c2.1, c2.2);

    // ------------------------------------------------

    let mut p = Person::new("Rohan", "Sood");
    println!("Name: {}", p.full_name());

    p.set_title("Dr.");
    println!("Name: {}", p.full_name());

    println!("Tuple: {:?}", p.to_tuple());
}
