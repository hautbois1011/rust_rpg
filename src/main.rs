mod attribute;
mod effect;
mod equipment;
mod unit;

use attribute::*;
use effect::*;
use equipment::*;
use unit::*;

fn main() {
    let mut u1 = Unit::new(1);
    GiveExp::new(1000).effect(&mut u1);
    ChangeHp::new(25, HpType::Damage).effect(&mut u1);
    ChangeHp::new(30, HpType::Recovery).effect(&mut u1);
    u1.set_attribute(Attribute::Rock);

    let eq1 = Equipment::new(Attribute::Rock, 4, 1, 0, 1);
    let eq2 = Equipment::new(Attribute::Scissors, 4, 0, 1, 1);
    u1.equip(eq1);
    u1.equip(eq2);
    println!("{:?}", u1);
}
