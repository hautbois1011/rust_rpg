#[derive(Debug, Clone, Copy)]
pub struct Unit {
    pub lv: u32,
    pub hp: u32,
    pub hp_max: u32,
    pub mp: u32,
    pub mp_max: u32,
    pub exp: u32,
}

impl Unit {
    pub fn new(lv: u32) -> Unit {
        Unit {
            lv: lv,
            hp: 20,
            hp_max: 20,
            mp: 20,
            mp_max: 20,
            exp: 0,
        }
    }

    pub fn update_exp_and_lv(&mut self, add: u32) {
        self.exp += add;

        let mut lv_changed = false;
        while self.exp >= self.lv * 100 {
            self.exp -= self.lv * 100;
            self.lv += 1;
            lv_changed = true;
        }

        if lv_changed {
            self.update();
        }
    }

    fn update(&mut self) {
        self.hp_max = 20 + (self.lv as f32).sqrt().floor() as u32;
        self.mp_max = 20 + ((self.lv as f32) / 5.0).floor() as u32;
    }
}
