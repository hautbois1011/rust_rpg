extern crate num;
use crate::unit::*;

pub trait Effect {
    fn effect(&self, opponent: &mut Unit);
}

pub struct GiveExp {
    exp: u32,
}

impl GiveExp {
    pub fn new(exp: u32) -> GiveExp {
        GiveExp { exp: exp }
    }
}

impl Effect for GiveExp {
    fn effect(&self, opponent: &mut Unit) {
        opponent.update_exp_and_lv(self.exp);
    }
}

pub struct ChangeHp {
    hp: i32,
}

impl ChangeHp {
    pub fn new(hp: i32) -> ChangeHp {
        ChangeHp { hp: hp }
    }
}

impl Effect for ChangeHp {
    fn effect(&self, opponent: &mut Unit) {
        opponent.hp = num::clamp(opponent.hp as i32 + self.hp, 0i32, opponent.hp_max as i32) as u32;
    }
}
