use crate::Pool;

#[derive(Debug, Copy, Clone)]
pub struct UnitDefinition {
    pub name: &'static str,
    pub cost: Pool,
    pub attack: u8,
    pub health: u8,
    pub speed: u8,
}

// Mono-element units
pub static MOSSBACK_TORTOISE: UnitDefinition = UnitDefinition {
    name: "Mossback Tortoise",
    cost: Pool {
        earth: 2,
        water: 0,
        fire: 0,
        air: 0,
    },
    attack: 1,
    health: 3,
    speed: 1,
};

pub static ROOTGUARD_DRYAD: UnitDefinition = UnitDefinition {
    name: "Rootguard Dryad",
    cost: Pool {
        earth: 4,
        water: 0,
        fire: 0,
        air: 0,
    },
    attack: 2,
    health: 5,
    speed: 1,
};

pub static GRANITE_BEHEMOTH: UnitDefinition = UnitDefinition {
    name: "Granite Behemoth",
    cost: Pool {
        earth: 6,
        water: 0,
        fire: 0,
        air: 0,
    },
    attack: 3,
    health: 7,
    speed: 1,
};

pub static ANCIENT_EARTH_TITAN: UnitDefinition = UnitDefinition {
    name: "Ancient Earth Titan",
    cost: Pool {
        earth: 8,
        water: 0,
        fire: 0,
        air: 0,
    },
    attack: 4,
    health: 9,
    speed: 1,
};

pub static TIDE_OTTER: UnitDefinition = UnitDefinition {
    name: "Tide Otter",
    cost: Pool {
        earth: 0,
        water: 2,
        fire: 0,
        air: 0,
    },
    attack: 2,
    health: 2,
    speed: 1,
};

pub static PEARL_NYMPH: UnitDefinition = UnitDefinition {
    name: "Pearl Nymph",
    cost: Pool {
        earth: 0,
        water: 4,
        fire: 0,
        air: 0,
    },
    attack: 4,
    health: 3,
    speed: 1,
};

pub static ABYSSAL_SERPENT: UnitDefinition = UnitDefinition {
    name: "Abyssal Serpent",
    cost: Pool {
        earth: 0,
        water: 6,
        fire: 0,
        air: 0,
    },
    attack: 6,
    health: 4,
    speed: 1,
};

pub static LEVIATHAN_AVATAR: UnitDefinition = UnitDefinition {
    name: "Leviathan Avatar",
    cost: Pool {
        earth: 0,
        water: 8,
        fire: 0,
        air: 0,
    },
    attack: 8,
    health: 5,
    speed: 1,
};

pub static EMBER_FOX: UnitDefinition = UnitDefinition {
    name: "Ember Fox",
    cost: Pool {
        earth: 0,
        water: 0,
        fire: 2,
        air: 0,
    },
    attack: 3,
    health: 1,
    speed: 1,
};

pub static CINDER_LYNX: UnitDefinition = UnitDefinition {
    name: "Cinder Lynx",
    cost: Pool {
        earth: 0,
        water: 0,
        fire: 4,
        air: 0,
    },
    attack: 5,
    health: 1,
    speed: 2,
};

pub static BLAZEHORN_CHIMERA: UnitDefinition = UnitDefinition {
    name: "Blazehorn Chimera",
    cost: Pool {
        earth: 0,
        water: 0,
        fire: 6,
        air: 0,
    },
    attack: 7,
    health: 1,
    speed: 3,
};

pub static INFERNAL_ELEMENTAL: UnitDefinition = UnitDefinition {
    name: "Infernal Elemental",
    cost: Pool {
        earth: 0,
        water: 0,
        fire: 8,
        air: 0,
    },
    attack: 9,
    health: 1,
    speed: 4,
};

pub static ZEPHYR_FALCON: UnitDefinition = UnitDefinition {
    name: "Zephyr Falcon",
    cost: Pool {
        earth: 0,
        water: 0,
        fire: 0,
        air: 2,
    },
    attack: 1,
    health: 1,
    speed: 3,
};

pub static GALE_SPRITE: UnitDefinition = UnitDefinition {
    name: "Gale Sprite",
    cost: Pool {
        earth: 0,
        water: 0,
        fire: 0,
        air: 4,
    },
    attack: 2,
    health: 1,
    speed: 5,
};

pub static STORMWING_ROC: UnitDefinition = UnitDefinition {
    name: "Stormwing Roc",
    cost: Pool {
        earth: 0,
        water: 0,
        fire: 0,
        air: 6,
    },
    attack: 4,
    health: 2,
    speed: 5,
};

pub static TEMPEST_ELEMENTAL: UnitDefinition = UnitDefinition {
    name: "Tempest Elemental",
    cost: Pool {
        earth: 0,
        water: 0,
        fire: 0,
        air: 8,
    },
    attack: 6,
    health: 3,
    speed: 5,
};

// Bi-element units: 2-2
pub static MUDROOT_GUARDIAN: UnitDefinition = UnitDefinition {
    name: "Mudroot Guardian",
    cost: Pool {
        earth: 2,
        water: 2,
        fire: 0,
        air: 0,
    },
    attack: 3,
    health: 4,
    speed: 1,
};

pub static ASHBARK_STAG: UnitDefinition = UnitDefinition {
    name: "Ashbark Stag",
    cost: Pool {
        earth: 2,
        water: 0,
        fire: 2,
        air: 0,
    },
    attack: 4,
    health: 3,
    speed: 1,
};

pub static STONEFEATHER_GRYPHON: UnitDefinition = UnitDefinition {
    name: "Stonefeather Gryphon",
    cost: Pool {
        earth: 2,
        water: 0,
        fire: 0,
        air: 2,
    },
    attack: 2,
    health: 3,
    speed: 3,
};

pub static STEAMFANG_OTTER: UnitDefinition = UnitDefinition {
    name: "Steamfang Otter",
    cost: Pool {
        earth: 0,
        water: 2,
        fire: 2,
        air: 0,
    },
    attack: 5,
    health: 2,
    speed: 1,
};

pub static MISTWING_HERON: UnitDefinition = UnitDefinition {
    name: "Mistwing Heron",
    cost: Pool {
        earth: 0,
        water: 2,
        fire: 0,
        air: 2,
    },
    attack: 3,
    health: 2,
    speed: 3,
};

pub static ASHEN_WYVERN: UnitDefinition = UnitDefinition {
    name: "Ashen Wyvern",
    cost: Pool {
        earth: 0,
        water: 0,
        fire: 2,
        air: 2,
    },
    attack: 4,
    health: 1,
    speed: 3,
};

// Bi-element units: 4-4
pub static DEEPWOOD_COLOSSUS: UnitDefinition = UnitDefinition {
    name: "Deepwood Colossus",
    cost: Pool {
        earth: 4,
        water: 4,
        fire: 0,
        air: 0,
    },
    attack: 5,
    health: 8,
    speed: 1,
};

pub static VOLCANIC_TREANT: UnitDefinition = UnitDefinition {
    name: "Volcanic Treant",
    cost: Pool {
        earth: 4,
        water: 0,
        fire: 4,
        air: 0,
    },
    attack: 7,
    health: 6,
    speed: 1,
};

pub static SKYROOT_ANCIENT: UnitDefinition = UnitDefinition {
    name: "Skyroot Ancient",
    cost: Pool {
        earth: 4,
        water: 0,
        fire: 0,
        air: 4,
    },
    attack: 4,
    health: 5,
    speed: 5,
};

pub static BOILING_LEVIATHAN: UnitDefinition = UnitDefinition {
    name: "Boiling Leviathan",
    cost: Pool {
        earth: 0,
        water: 4,
        fire: 4,
        air: 0,
    },
    attack: 8,
    health: 4,
    speed: 2,
};

pub static CLOUDTIDE_ORACLE: UnitDefinition = UnitDefinition {
    name: "Cloudtide Oracle",
    cost: Pool {
        earth: 0,
        water: 4,
        fire: 0,
        air: 4,
    },
    attack: 5,
    health: 4,
    speed: 5,
};

pub static THUNDERFLAME_PHOENIX: UnitDefinition = UnitDefinition {
    name: "Thunderflame Phoenix",
    cost: Pool {
        earth: 0,
        water: 0,
        fire: 4,
        air: 4,
    },
    attack: 7,
    health: 2,
    speed: 5,
};

// Tri-element units: 4-2-2
pub static VERDANT_TIDEWARDEN: UnitDefinition = UnitDefinition {
    name: "Verdant Tidewarden",
    cost: Pool {
        earth: 4,
        water: 2,
        fire: 2,
        air: 0,
    },
    attack: 6,
    health: 7,
    speed: 1,
};

pub static MIREFLAME_HYDRA: UnitDefinition = UnitDefinition {
    name: "Mireflame Hydra",
    cost: Pool {
        earth: 2,
        water: 4,
        fire: 2,
        air: 0,
    },
    attack: 8,
    health: 5,
    speed: 1,
};

pub static CINDERROOT_JUGGERNAUT: UnitDefinition = UnitDefinition {
    name: "Cinderroot Juggernaut",
    cost: Pool {
        earth: 2,
        water: 2,
        fire: 4,
        air: 0,
    },
    attack: 8,
    health: 5,
    speed: 1,
};

pub static ROOTSTORM_SENTINEL: UnitDefinition = UnitDefinition {
    name: "Rootstorm Sentinel",
    cost: Pool {
        earth: 4,
        water: 2,
        fire: 0,
        air: 2,
    },
    attack: 4,
    health: 7,
    speed: 3,
};

pub static RAINSPIRE_SPIRIT: UnitDefinition = UnitDefinition {
    name: "Rainspire Spirit",
    cost: Pool {
        earth: 2,
        water: 4,
        fire: 0,
        air: 2,
    },
    attack: 6,
    health: 5,
    speed: 3,
};

pub static GROVEWIND_ELDER: UnitDefinition = UnitDefinition {
    name: "Grovewind Elder",
    cost: Pool {
        earth: 2,
        water: 2,
        fire: 0,
        air: 4,
    },
    attack: 4,
    health: 5,
    speed: 5,
};

pub static BASALT_STORMCALLER: UnitDefinition = UnitDefinition {
    name: "Basalt Stormcaller",
    cost: Pool {
        earth: 4,
        water: 0,
        fire: 2,
        air: 2,
    },
    attack: 5,
    health: 6,
    speed: 3,
};

pub static WILDFIRE_COLOSSUS: UnitDefinition = UnitDefinition {
    name: "Wildfire Colossus",
    cost: Pool {
        earth: 2,
        water: 0,
        fire: 4,
        air: 2,
    },
    attack: 8,
    health: 3,
    speed: 3,
};

pub static TEMPEST_GROVE_WYVERN: UnitDefinition = UnitDefinition {
    name: "Tempest Grove Wyvern",
    cost: Pool {
        earth: 2,
        water: 0,
        fire: 2,
        air: 4,
    },
    attack: 5,
    health: 4,
    speed: 5,
};

pub static SCALDING_MISTCALLER: UnitDefinition = UnitDefinition {
    name: "Scalding Mistcaller",
    cost: Pool {
        earth: 0,
        water: 4,
        fire: 2,
        air: 2,
    },
    attack: 8,
    health: 3,
    speed: 3,
};

pub static SUNKEN_FLAME_EIDOLON: UnitDefinition = UnitDefinition {
    name: "Sunken Flame Eidolon",
    cost: Pool {
        earth: 0,
        water: 2,
        fire: 4,
        air: 2,
    },
    attack: 8,
    health: 2,
    speed: 4,
};

pub static SKYCURRENT_SERAPH: UnitDefinition = UnitDefinition {
    name: "Skycurrent Seraph",
    cost: Pool {
        earth: 0,
        water: 2,
        fire: 2,
        air: 4,
    },
    attack: 6,
    health: 3,
    speed: 5,
};

// Quadri-element unit
pub static CHROMATIC_AVATAR: UnitDefinition = UnitDefinition {
    name: "Chromatic Avatar",
    cost: Pool {
        earth: 2,
        water: 2,
        fire: 2,
        air: 2,
    },
    attack: 5,
    health: 5,
    speed: 4,
};

pub static UNIT_CATALOG: [&'static UnitDefinition; 41] = [
    &MOSSBACK_TORTOISE,
    &ROOTGUARD_DRYAD,
    &GRANITE_BEHEMOTH,
    &ANCIENT_EARTH_TITAN,
    &TIDE_OTTER,
    &PEARL_NYMPH,
    &ABYSSAL_SERPENT,
    &LEVIATHAN_AVATAR,
    &EMBER_FOX,
    &CINDER_LYNX,
    &BLAZEHORN_CHIMERA,
    &INFERNAL_ELEMENTAL,
    &ZEPHYR_FALCON,
    &GALE_SPRITE,
    &STORMWING_ROC,
    &TEMPEST_ELEMENTAL,
    &MUDROOT_GUARDIAN,
    &ASHBARK_STAG,
    &STONEFEATHER_GRYPHON,
    &STEAMFANG_OTTER,
    &MISTWING_HERON,
    &ASHEN_WYVERN,
    &DEEPWOOD_COLOSSUS,
    &VOLCANIC_TREANT,
    &SKYROOT_ANCIENT,
    &BOILING_LEVIATHAN,
    &CLOUDTIDE_ORACLE,
    &THUNDERFLAME_PHOENIX,
    &VERDANT_TIDEWARDEN,
    &MIREFLAME_HYDRA,
    &CINDERROOT_JUGGERNAUT,
    &ROOTSTORM_SENTINEL,
    &RAINSPIRE_SPIRIT,
    &GROVEWIND_ELDER,
    &BASALT_STORMCALLER,
    &WILDFIRE_COLOSSUS,
    &TEMPEST_GROVE_WYVERN,
    &SCALDING_MISTCALLER,
    &SUNKEN_FLAME_EIDOLON,
    &SKYCURRENT_SERAPH,
    &CHROMATIC_AVATAR,
];