enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Result::Err(String::from("Denominator cannot be zero"))
    } else {
        Result::Ok(numerator / denominator)
    }
}

fn main() {
    match divide(2.0, 1.0) {
        Result::Ok(result) => println!("Result: {}", result),
        Result::Err(e) => println!("Error: {}", e),
    }
}

