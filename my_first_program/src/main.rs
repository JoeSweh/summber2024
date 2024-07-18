
const FREEZING_POINT:f32 = 32.0;

fn fahrenheit_to_celsius(temperature:f64) -> f64 {
    temperature - (FREEZING_POINT as f64) * (5.0/9.0) as f64
}


fn main(){
    println!("{}",FREEZING_POINT);
    let farenhait = 100.0;

    let in_celsius = fahrenheit_to_celsius(farenhait);

    let temperature: [i32; 5] =[33,34,35,36,37];

    println!("{}", in_celsius);
}