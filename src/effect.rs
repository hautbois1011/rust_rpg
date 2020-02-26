extern crate num;
use crate::unit::*;

pub trait Effect {
    fn effect(&self, opponent: &mut Unit);
}

//----------------------------------------------------

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

//----------------------------------------------------

pub enum HpType {
    Recovery,
    Damage,
}

pub struct ChangeHp {
    hp: u32,
    mode: HpType,
}

impl ChangeHp {
    pub fn new(hp: u32, mode: HpType) -> ChangeHp {
        ChangeHp { hp: hp, mode: mode }
    }
}

impl Effect for ChangeHp {
    fn effect(&self, opponent: &mut Unit) {
        opponent.hp = match self.mode {
            HpType::Recovery => num::clamp(opponent.hp + self.hp, 0, opponent.hp_max),
            HpType::Damage => num::clamp(
                opponent.hp as i32 - self.hp as i32,
                0,
                opponent.hp_max as i32,
            ) as u32,
        }
    }
}
