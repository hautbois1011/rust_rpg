#[derive(Debug, Clone, Copy)]
pub enum Attribute {
    NoAttribute,
    Rock,
    Scissors,
    Paper,
}

#[derive(Debug, Clone, Copy)]
pub struct Equipment {
    pub attribute: Attribute,
    pub firepower: i32,
    pub endurance: i32,
    pub accuracy: i32,
    pub avoidance: i32,
}

impl Equipment {
    pub fn new(attr: Attribute, fire: i32, endr: i32, accr: i32, avd: i32) -> Equipment {
        Equipment {
            attribute: attr,
            firepower: fire,
            endurance: endr,
            accuracy: accr,
            avoidance: avd,
        }
    }
}
