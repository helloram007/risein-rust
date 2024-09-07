fn main() {
    let mut vec = vec![1,2,3,4,5];

    for item in vec.into_iter() {
        //for item in vec.iter_mut() {
        //*item *= 2;
        println!("{}", item);
    }
}

