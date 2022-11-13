use text_io::read;

mod convert;

fn main() {
    println!("Hello, welcome to cash converter, the number of coins specified will determine the value!");
    println!("Quarters: ");
    let quarters: i32 = read!();
    println!("Dimes: ");
    let dimes: i32 = read!();
    println!("Nickles: ");
    let nickles: i32 = read!();
    println!("Pennies: ");
    let pennies: i32 = read!();

    convert::print_conversion(quarters, dimes, nickles, pennies);

}
