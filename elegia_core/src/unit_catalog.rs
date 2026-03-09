use crate::{Pool};

#[derive(Debug)]
pub struct UnitDefinition {
    pub name: &'static str,
    pub cost: Pool,
    pub attack: u8,
    pub health: u8,
    pub speed: u8,
}

pub const UNIT_CATALOG: [UnitDefinition; 3] = [
    UnitDefinition {
        name: "Mossback Tortoise",
        cost: Pool { earth: 2, water: 0, fire: 0, air: 0 },
        attack: 1,
        health: 5,
        speed: 1,
    },
    UnitDefinition {
        name: "Ember Fox",
        cost: Pool { earth: 0, water: 0, fire: 2, air: 0 },
        attack: 3,
        health: 2,
        speed: 2,
    },
    UnitDefinition {
        name: "Zephyr Falcon",
        cost: Pool { earth: 0, water: 0, fire: 0, air: 2 },
        attack: 2,
        health: 2,
        speed: 3,
    },
];

pub fn find_unit_by_name(name: &str) -> Option<&'static UnitDefinition> {
    UNIT_CATALOG.iter().find(|unit| unit.name == name)
}