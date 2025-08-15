// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

// * Use an enum to create different flavors of drinks
enum Flavor {
    Vanilla,
    Strawberry,
    Chocolate,
}
// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}
// * Use a function to print out the drink flavor and ounces
fn print_flavor(drink: Drink) {
    match drink.flavor {
        Flavor::Chocolate => println!("Chocolate"),
        Flavor::Strawberry => println!("Strawberry"),
        Flavor::Vanilla => println!("Vanilla"),
    }
    println!("oz: {:?}", drink.fluid_oz);
}
fn main() {
    let sweet = Drink {
        flavor: Flavor::Chocolate,
        fluid_oz: 6.8,
    };
    print_flavor(sweet);
    let fruity = Drink {
        flavor: Flavor::Strawberry,
        fluid_oz: 9.7,
    };
    print_flavor(fruity);
    let icy = Drink {
        flavor: Flavor::Vanilla,
        fluid_oz: 5.6,
    };
    print_flavor(icy);
}
