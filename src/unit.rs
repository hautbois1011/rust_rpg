use crate::equipment::*;

#[derive(Debug, Clone)]
pub struct Unit {
    pub lv: u32,
    pub hp: u32,
    pub hp_max: u32,
    pub mp: u32,
    pub mp_max: u32,
    pub exp: u32,

    pub equipment_list: Vec<Equipment>,
}

impl Unit {
    pub fn new(lv: u32) -> Unit {
        let mut u = Unit {
            lv: lv,
            hp: 0,
            hp_max: 0,
            mp: 0,
            mp_max: 0,
            exp: 0,
            equipment_list: Vec::new(),
        };
        u.update();
        u
    }

    pub fn update_exp_and_lv(&mut self, add: u32) {
        self.exp += add;

        let mut lv_changed = false;
        while self.exp >= self.lv * 100 {
            self.exp -= self.lv * 100;
            self.lv += 1;
            lv_changed = true;
        }

        if self.lv > 99 {
            self.lv = 99;
        }

        if lv_changed {
            self.update();
        }
    }

    pub fn equip(&mut self, equipment: Equipment) {
        self.equipment_list.push(equipment);
    }

    fn update(&mut self) {
        self.hp_max = 20 + (self.lv as f32).sqrt().floor() as u32;
        self.mp_max = 20 + ((self.lv as f32) / 5.0).floor() as u32;
    }
}
