use crate::attribute::*;
use crate::equipment::*;

#[derive(Debug, Clone)]
pub struct Unit {
    lv: u32,
    hp: u32,
    hp_max: u32,
    mp: u32,
    mp_max: u32,
    exp: u32,

    attribute: Attribute,
    equipment_list: Vec<Equipment>,
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
            attribute: Attribute::NoAttribute,
            equipment_list: Vec::new(),
        };
        u.update();
        u.fullfill();
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

    // getter / setter

    pub fn attribute(&self) -> Attribute {
        self.attribute
    }

    pub fn set_attribute(&mut self, attr: Attribute) {
        self.attribute = attr;
    }

    pub fn hp(&self) -> u32 {
        self.hp
    }

    pub fn set_hp(&mut self, hp: u32) {
        self.hp = hp;
    }

    pub fn hp_max(&self) -> u32 {
        self.hp_max
    }

    // helper function

    fn update(&mut self) {
        self.hp_max = 20 + (self.lv as f32).sqrt().floor() as u32;
        self.mp_max = 20 + ((self.lv as f32) / 5.0).floor() as u32;
    }

    fn fullfill(&mut self) {
        self.hp = self.hp_max;
        self.mp = self.mp_max;
    }
}
