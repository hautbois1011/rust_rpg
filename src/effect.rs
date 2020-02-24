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
