mod effect;
mod equipment;
mod unit;

use effect::*;
use equipment::*;
use unit::*;

fn main() {
    let mut u1 = Unit::new(1);
    GiveExp::new(1000).effect(&mut u1);
    println!("{:?}", u1);
    ChangeHp::new(25, HpType::Damage).effect(&mut u1);
    println!("{:?}", u1);
    ChangeHp::new(30, HpType::Recovery).effect(&mut u1);
    println!("{:?}", u1);

    let eq1 = Equipment::new(Attribute::Rock, 4, 1, 0, 1);
    let eq2 = Equipment::new(Attribute::Scissors, 4, 0, 1, 1);
    u1.equip(eq1);
    u1.equip(eq2);
    println!("{:?}", u1);
}
