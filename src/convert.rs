mod quarters;
mod dimes;
mod nickles;
mod pennies; 

fn money_string(amount: f64) -> String {
    format!("${:.2}", amount)
}

fn convert_to_value(in_quarters: i32, in_dimes: i32, in_nickles: i32, in_pennies: i32) -> f64{
    return quarters::convert(in_quarters) + dimes::convert(in_dimes) + nickles::convert(in_nickles)+ pennies::convert(in_pennies);
}

pub fn print_conversion(in_quarters: i32, in_dimes: i32, in_nickles: i32, in_pennies: i32) {

    let converted_value = money_string(convert_to_value(in_quarters, in_dimes, in_nickles, in_pennies));
    println!("You desposited {quarters} quarters, {dimes} dimes, {nickles} nickles, {pennies} pennies which is {formatted_output} usd", 
        quarters = in_quarters,
        dimes = in_dimes,
        nickles = in_nickles,
        pennies = in_pennies,
        formatted_output = converted_value);
}