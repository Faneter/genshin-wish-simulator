use crate::simulator::{StarSimulator, UpSimulator};

pub trait Strategy {
    fn run<T: UpSimulator>(&mut self,up: &mut T);
}

pub struct Strategy1 {
    counts_from_last_3: i32,
    star: StarSimulator,
    money: i32,
}

impl Strategy1 {
    pub fn new(m: i32) -> Strategy1 {
        Strategy1 { 
            counts_from_last_3: 0,
            star: StarSimulator::new(),
            money: m,
        }
    }

    fn next_step(&mut self, money:i32) -> bool {
        if money < 10 {
            return false;
        }
        return self.counts_from_last_3 == 8
    }
    
    fn set_result(&mut self, result: i32) {
        if result == 3 {
            self.counts_from_last_3 += 1
        } else {
            self.counts_from_last_3 = 0;
        }
    }
}

impl Strategy for Strategy1 {
    fn run<T: UpSimulator>(&mut self,up: &mut T) {
        while self.money > 0 {
            if self.next_step(self.money) {
                self.money -= 10;
                for i in self.star.wish_10(up).iter() {
                    self.set_result(*i);
                }
            } else {
                self.money -= 1;
                let i = self.star.wish(up);
                self.set_result(i);
            }
        }
    }
}
