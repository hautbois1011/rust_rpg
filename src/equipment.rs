use crate::attribute::*;

#[derive(Debug, Clone, Copy)]
pub struct Equipment {
    attribute: Attribute,
    firepower: i32,
    endurance: i32,
    accuracy: i32,
    avoidance: i32,
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

    pub fn attribute(&self) -> Attribute {
        self.attribute
    }

    pub fn firepower(&self) -> i32 {
        self.firepower
    }

    pub fn endurance(&self) -> i32 {
        self.endurance
    }

    pub fn accuracy(&self) -> i32 {
        self.accuracy
    }

    pub fn avoidance(&self) -> i32 {
        self.avoidance
    }
}
