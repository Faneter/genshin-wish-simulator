use rand::Rng;

/**
 * 用于生成物品星级
 * 
 * TODO 配合UP模拟器判断UP情况
 * 
 * TODO 配合数据分析器分析总体情况
 */
pub struct StarSimulator {
    pub counts: i32,
    pub counts_from_last_5: i32,
    pub counts_from_last_4: i32,
}

impl StarSimulator {
    pub fn new() -> StarSimulator {
        StarSimulator { counts: 0, counts_from_last_5: 0, counts_from_last_4: 0 }
    }

    pub fn wish(&mut self) -> i32 {
        self.counts += 1;
        let seed = rand::rng().random_range(1..=1000);
        if self.counts_from_last_5 <= 73 && seed <= 6 || seed <= (6 + 60*(self.counts_from_last_5 - 73)) {
            println!("{}抽出金", self.counts_from_last_5 + 1);
            self.counts_from_last_5 = 0;
            return 5;
        }
        self.counts_from_last_5 += 1;
        let seed = rand::rng().random_range(1..=1000);
        if (self.counts_from_last_4 <= 7 && seed <= 51) || (self.counts_from_last_4 == 8 && seed <= 51) || self.counts_from_last_4 == 9 {
            // println!("{}抽出四星", self.counts_from_last_5 + 1);
            self.counts_from_last_4 = 0;
            return 4;
        }
        self.counts_from_last_4 += 1;
        return 3;
    }

    pub fn wish_10(&mut self) -> [i32; 10] {
        return [self.wish(), self.wish(), self.wish(), self.wish(), self.wish(),
        self.wish(),self.wish(),self.wish(),self.wish(),self.wish()];
    }


}

pub struct CharacterUpSimulator {
    pub light_count: i32,
    last_up: bool, // 小保底歪了没
}

impl CharacterUpSimulator {
    pub fn new() -> CharacterUpSimulator {
        CharacterUpSimulator {
            light_count: 1,
            last_up: false,
        }
    }

    pub fn simulate(&mut self) -> bool {
        if self.last_up == true {
            return true;
        }
        if self.light_count == 3 {
            self.light_count = 1;
            return true;
        }
        let seed = rand::rng().random_range(1..=100);
        if self.light_count == 2 && seed <=75 {
            self.light_count = 1;
            return true;
        }
        let seed = rand::rng().random_range(1..=100);
        if self.light_count == 1 && seed <= 50 {
            self.light_count = 0;
            return true;
        } else if self.light_count == 0 && seed <= 50 {
            return true;
        }
        self.light_count += 1;
        self.last_up = true;
        return false;
    }
}