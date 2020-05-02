use std::io;

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {

    let mut height = String::new();
    let mut width = String::new();

    println!("Insert height of a rectangle and press ENTER: ");

    io::stdin()
        .read_line(&mut height)
        .expect("Failed to read line");

    let height: u32 = from_string_to_int(height);

    println!("Insert width of a rectangle and press ENTER: ");

    io::stdin()
        .read_line(&mut width)
        .expect("Failed to read line");

    let width: u32 = from_string_to_int(width);

    let rectangle = Rectangle {
        width,
        height,
    };

    println!("The area of a rectangle is {}", rectangle.area());
}



fn from_string_to_int(input: String) -> u32 {
    input.trim()
        .parse()
        .expect("Please insert a number")
}
