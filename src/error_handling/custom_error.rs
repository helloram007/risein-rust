fn main()  {
    let my_error = RockerError::AlienInvasion;
    handle_error(my_error);

}

fn divide(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
    if denominator == 0.0 {
        Err("Cannot divide by zero.")
    } else {
        Ok (numerator/denominator)
    }
}

enum RockerError {
    OutOfFuel,
    NavigationSystemFailure,
    AlienInvasion,
}

fn handle_error(error: RockerError) {
    match error {
        RockerError::OutOfFuel => {
            println!("Houston, we dont have any fuel, come and get us");
        }
        RockerError::NavigationSystemFailure => {
            println!("we dont know where to go houston, our navigation has failed");
        }
        RockerError::AlienInvasion => {
            println!("i got couple of Aliens here. Do you have any questions Houston?");
        }
    }
}