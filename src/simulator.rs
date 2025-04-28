use rand::Rng;

use crate::analysis;

/**
 * 用于生成物品星级
 *
 * 配合UP模拟器判断UP情况
 *
 * TODO 配合数据分析器分析总体情况
 */
pub struct StarSimulator {
    counts_from_last_5: i32,
    counts_from_last_4: i32,
}

impl StarSimulator {
    pub fn new(dian: i32) -> StarSimulator {
        StarSimulator {
            counts_from_last_5: dian,
            counts_from_last_4: 0,
        }
    }

    pub fn wish<T: UpSimulator>(&mut self, up: &mut T) -> i32 {
        let seed = rand::rng().random_range(1..=1000);
        if self.counts_from_last_5 <= 73 && seed <= 6
            || seed <= (6 + 60 * (self.counts_from_last_5 - 73))
        {
            if up.simulate() {
                println!("第{}抽出金，为UP金", self.counts_from_last_5 + 1);
                analysis::add(self.counts_from_last_5, true);
            } else {
                println!("第{}抽出金，并歪了", self.counts_from_last_5 + 1);
                analysis::add(self.counts_from_last_5, false);
            }
            self.counts_from_last_5 = 0;
            return 5;
        }
        self.counts_from_last_5 += 1;
        let seed = rand::rng().random_range(1..=1000);
        if (self.counts_from_last_4 <= 7 && seed <= 51)
            || (self.counts_from_last_4 == 8 && seed <= 563)
            || self.counts_from_last_4 == 9
        {
            // println!("第{}抽出四星", self.counts_from_last_5 + 1);
            self.counts_from_last_4 = 0;
            return 4;
        }
        self.counts_from_last_4 += 1;
        return 3;
    }

    pub fn wish_10<T: UpSimulator>(&mut self, up: &mut T) -> [i32; 10] {
        let mut arr: [i32; 10] = [0; 10];
        for i in 0..arr.len() {
            arr[i] = self.wish(up);
        }
        return arr;
    }
}
pub trait UpSimulator {
    fn simulate(&mut self) -> bool {
        return true;
    }
}

pub struct CharacterUpSimulator {
    pub light_count: i32, // 捕获明光计数器
    last_up: bool,        // 上次是小保底的同时歪了没
}

impl CharacterUpSimulator {
    pub fn new(light: i32, wai: bool) -> CharacterUpSimulator {
        CharacterUpSimulator {
            light_count: light,
            last_up: wai,
        }
    }
}
impl UpSimulator for CharacterUpSimulator {
    fn simulate(&mut self) -> bool {
        if self.last_up == true {
            self.last_up = false;
            return true;
        }
        if self.light_count == 3 {
            self.light_count = 1;
            println!("上面触发捕获明光");
            return true;
        }
        let seed = rand::rng().random_range(1..=100);
        if self.light_count == 2 && seed <= 50 {
            self.light_count = 1;
            return true;
        }
        let seed = rand::rng().random_range(1..=100);
        if self.light_count == 1 && seed <= 50 {
            self.light_count = 0;
            return true;
        }
        let seed = rand::rng().random_range(1..=100);
        if self.light_count == 0 && seed <= 50 {
            return true;
        }
        self.light_count += 1;
        self.last_up = true;
        return false;
    }
}
