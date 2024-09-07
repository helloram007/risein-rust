fn main() {

    let countdown = Countdown { remaininng: 5 };

    for i in countdown {
        println!("{}", i);
    }

}

struct Countdown {
    remaininng: i32,
}

impl Iterator for Countdown {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaininng > 0 {
            let current = self.remaininng;
            self.remaininng -= 1;
            Some(current)
        } else {
            None
        }
    }
}