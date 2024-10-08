fn main() {

    let mut fibonacci = Fibonacci {
        current: 0,
        next: 1,
    };

    for _ in 0..10 {
        println!("{}", fibonacci.next().unwrap());
    }

}

struct Fibonacci {
    current: u32,
    next: u32,
}

impl Iterator for Fibonacci {

    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.next;
        self.next = current + self.next;

        Some(current)
    }
}