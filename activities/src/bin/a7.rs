// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print
#[derive(Copy, Clone)]
enum Colors{
    Red,
    Blue,
    Green,
    Orange,
    White,
    Pink,
    Purple,
    Violet
}
fn print(color: Colors) {
    match color {
        Colors::Red => println!("Red"),
        Colors::Green => println!("Green"),
        Colors::Blue => println!("Blue"),
        Colors::Orange => println!("Orange"),
        Colors::White => println!("White"),
        Colors::Pink => println!("Pink"),
        Colors::Purple => println!("Purple"),
        Colors::Violet => println!("Violet"),
    }
}

fn main() {
    let colors = [
        Colors::Red,
        Colors::Blue,
        Colors::Green,
        Colors::Orange,
        Colors::White,
        Colors::Pink,
        Colors::Purple,
        Colors::Violet,
    ];
    for color in colors.iter() {
        print(*color);
    }
}

