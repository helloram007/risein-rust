use std::io;

fn main() {
    
   // Create a mutable string to hold the user's input
   let mut input = String::new();
   println!("Please enter value x as a floating number");
   // read input from the user
   io::stdin().read_line(&mut input).expect("Failed to read line for X");

   // trim the input to remove any extra whitespace and attempt to parse it to f64
   let x: f64 = input.trim().parse().expect("Please enter a valid floadting-point number");


   // please enter value Y
   let mut input = String::new();
   println!("Please enter value Y as a floating number");
   // read input from the user for value Y
   io::stdin().read_line(&mut input).expect("Failed to read line for Y");

   // Trim the input to remove any extra whitespaces and attempt to parse it to f64
   let y: f64 = input.trim().parse().expect("Failed to enter a valid floadting-point number");


    let add = Operation::Add {x,y};
    let add_result = add.calculate();
    println!("Result: {}", add_result);
}


enum Operation {
    Add {x: f64, y: f64},
    Subtract {x: f64, y: f64},
    Multiply {x: f64, y: f64},
    Divide {x: f64, y: f64},
}

impl Operation {
    fn calculate(&self) -> f64 {
        match self {
            Operation::Add { x,y }=> x + y ,
            Operation::Subtract { x,y } => x - y,
            Operation::Multiply { x,y } => x * y,
            Operation::Divide { x,y } => {
                if *y > 0.0 {
                    x / y
                } else {
                    0.0
                }
            }
        }
    }
}