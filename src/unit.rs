#[derive(Debug, Clone, Copy)]
pub struct Unit {
    lv: u32,
    hp: u32,
    mp: u32,
    exp: u32,
}

impl Unit {
    pub fn new(lv: u32) -> Unit {
        Unit {
            lv: lv,
            hp: 20,
            mp: 20,
            exp: 0,
        }
    }

    pub fn update_exp_and_lv(&mut self, add: u32) {
        self.exp += add;

        while self.exp >= self.lv * 100 {
            println!("{:?}", self.exp);
            self.exp -= self.lv * 100;
            self.lv += 1;
        }
    }
}
