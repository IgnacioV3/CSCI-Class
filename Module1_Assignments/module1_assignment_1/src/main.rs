// Freeze point constant (°F)
const FRZ_PT:f64 = 32.0;

// Conversion functions
fn fahrenheit_to_celsius(f: f64) -> f64{
    (f - FRZ_PT) * 5.0 / 9.0 
}

fn celsius_to_fahrenheit(c: f64) -> f64{
    (c * 1.8) + FRZ_PT
}



fn main() {
    // Assignment 1: Temperature Converter

    // Mutable temp variable
    let mut temp_f:f64 = 72.0;

    // Converting to celsius
    temp_f = fahrenheit_to_celsius(temp_f);
    println!("{}°F = {:.2}°C", celsius_to_fahrenheit(temp_f) , temp_f);

    // For loop to convert celsius next 5 temps
    let start_f = celsius_to_fahrenheit(temp_f) as i32 + 1;
    
    for f in start_f..start_f + 5{
        let c = fahrenheit_to_celsius(f as f64);
        println!("{}°F = {:.2}°C", f , c);
}

}
