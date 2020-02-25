mod effect;
mod unit;

use effect::*;
use unit::*;

fn main() {
    let mut u1 = Unit::new(1);
    let exp_effect = GiveExp::new(1000);
    exp_effect.effect(&mut u1);
    println!("{:?}", u1);
    ChangeHp::new(-25).effect(&mut u1);
    println!("{:?}", u1);
}
