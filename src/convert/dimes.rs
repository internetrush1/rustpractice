const CONVERT_VALUE: f64 = 0.10; 

pub fn convert(in_value: i32) -> f64 {
    println!("Converted {coins} Dimes to: {convert}", coins= in_value, convert = CONVERT_VALUE);
    return in_value as f64 * CONVERT_VALUE;
}