fn main() {
    let int_container = Container { value: 42 };
    let str_container = Container { value: "Simon" };
    println!("{:?}", int_container);
    println!("{:?}", str_container);

    let success_result: MagicalResult<i32, String> = MagicalResult::Success(42);
    let failure_result: MagicalResult<i32, String> = MagicalResult::Failure("it failed".to_string());

    match success_result  {
        MagicalResult::Success(value) => println!("we got success {}", value),
        MagicalResult::Failure(value) => println!("we got failure {}", value),
    }

    match failure_result  {
        MagicalResult::Success(value) => println!("we got success {}", value),
        MagicalResult::Failure(value) => println!("we got failure {}", value),
    }
}

#[derive(Debug)]
struct Container<T> {
    value: T,
}

enum MagicalResult<T,E> {
    Success(T),
    Failure(E),
}
