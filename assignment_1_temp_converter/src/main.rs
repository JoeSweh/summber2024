const FREEZING_POINT:f64 = 32.0;

fn fahrenheit_to_celsius(f:f64) -> f64 {
    (f - FREEZING_POINT) * (5.0/9.0)
}

fn celsius_to_fahrenheit(c: f64) -> f64{
    (c * (9.0/5.0)) + FREEZING_POINT
}

fn main() {
    println!("Name: Jose F. Gonzalez Jr.");
    let mut fahrenheit = 32.0;

    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{}°F is {:.2}°C", fahrenheit, celsius);

    for _ in 0..5 {
        fahrenheit += 1.0;
        let celsius = fahrenheit_to_celsius(fahrenheit);
        println!("{}°F is {:.2}°C", fahrenheit, celsius);
    }

}