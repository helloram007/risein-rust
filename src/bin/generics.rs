use std::{collections::hash_map::IterMut, iter::Sum};

fn main() {

    
}

fn swap<T: Copy>(x: &mut T, y: &mut T) {
    let temp = *x;
    *x = *y;
    *y = temp;
}

trait Summary {
    fn summarize(&self);
}

fn print_summary<T: Summary>(item: T) {
    println!("{:?}", item.summarize());
}

fn print_double_summary<T, U>(item1: T, item2: U)
where 
    T: Summary,
    U: Summary + Clone,
{
    println!("{:?}",item1.summarize());
    println!("{:?}",item2.summarize());
    let cloned_item = item2.clone();
    println!("{:?}", cloned_item.summarize());
}

