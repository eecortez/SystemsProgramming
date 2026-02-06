const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn main() {
    let mut temperature_f: f64 = FREEZING_POINT_F;

    let temperature_c = fahrenheit_to_celsius(temperature_f);
    println!("{}°F is equal to {}°C", temperature_f, temperature_c);

    for i in 1..=5 {
        temperature_f = FREEZING_POINT_F + i as f64;
        let temp_c = fahrenheit_to_celsius(temperature_f);
        println!("{}°F is equal to {}°C", temperature_f, temp_c);
    }
}

