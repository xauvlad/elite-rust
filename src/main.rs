fn divide_numbers(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        return Err("Division by zero is not allowed");
    }
    else if b < 0.0 {
        return Err("Divisor is negative");
    }
    if a < 0.0 {
        return Err("Divider is negative");
    }
    Ok(a / b)
}

fn main() {
    let x = 10.1;
    let y = 2.0;

    let result = divide_numbers(x, y);
    
    match result {
        Ok(value) => println!("First result: {:?}", value),
        Err(e) => println!("An error occurred: {}", e),
    }

    let result_2 = divide_numbers(7.0, 0.0);
    // if let Ok(value) = result_2 {
    //     println!("Second result: {}", value);
    // }
    match result_2 {
        Ok(value) => println!("Second result: {:?}", value),
        Err(e) => println!("An error occurred: {}", e),
    }

    let result_3 = divide_numbers(-5.0, 2.5);

    match result_3 {
        Ok(value) => println!("Third result: {:?}", value),
        Err(e) => println!("An error occurred: {}", e),
    }

    let result_4 = divide_numbers(5.0, -2.5);

    match result_4 {
        Ok(value) => println!("Fourth result: {:?}", value),
        Err(e) => println!("An error occurred: {}", e),
    }
}