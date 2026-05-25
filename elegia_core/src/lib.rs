pub mod unit_catalog;
use crate::unit_catalog::UnitDefinition;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Element {
    Fire,
    Water,
    Air,
    Earth,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Pool {
    pub fire: u8,
    pub water: u8,
    pub air: u8,
    pub earth: u8,
}

impl Pool {
    fn up(&mut self, element: Element) {
        match element {
            Element::Fire => self.fire += 1,
            Element::Water => self.water += 1,
            Element::Air => self.air += 1,
            Element::Earth => self.earth += 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hex {
    pub q: i8,
    pub r: i8,
}

#[derive(Debug, Copy, Clone)]
pub struct Board {
    radius: u8,
}

impl Default for Board {
    fn default() -> Self {
        Self { radius: 4 }
    }
}

impl Board {
    pub fn all_hexes(&self) -> Vec<Hex> {
        let radius = self.radius as i8;
        let mut hexes = Vec::new();

        for q in -radius..=radius {
            for r in -radius..=radius {
                let hex = Hex { q, r };

                if self.is_valid_hex(hex) {
                    hexes.push(hex);
                }
            }
        }

        hexes
    }

    fn is_valid_hex(&self, hex: Hex) -> bool {
        let q = hex.q;
        let r = hex.r;
        let s = -q - r;

        q.abs().max(r.abs()).max(s.abs()) <= self.radius.try_into().unwrap()
    }

    pub fn orb_hex(&self, player_id: PlayerId) -> Hex {
        match player_id {
            PlayerId::South => Hex { q: -2, r: 4 },
            PlayerId::North => Hex { q: 2, r: -4 },
        }
    }

    pub fn is_spawn_hex(&self, hex: Hex, player_id: PlayerId) -> bool {
        if !self.is_valid_hex(hex) || hex == self.orb_hex(player_id) {
            return false;
        }

        match player_id {
            PlayerId::South => hex.r >= 2 && hex.r <= 4,
            PlayerId::North => hex.r >= -4 && hex.r <= -2,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Unit {
    id: u16,
    pub kind: &'static UnitDefinition,
    pub position: Hex,
    pub current_attack: u8,
    pub current_health: u8,
    pub current_speed: u8,
}

impl Unit {
    fn take_damage(&mut self, amount: u8) -> bool {
        if amount >= self.current_health {
            self.current_health = 0;
            return true;
        }

        self.current_health -= amount;
        false
    }
}

#[derive(Debug, Copy, Clone)]
struct Orb {
    max_health: u8,
    current_health: u8,
}

impl Default for Orb {
    fn default() -> Self {
        Self {
            max_health: 25,
            current_health: 25,
        }
    }
}

impl Orb {
    fn take_damage(&mut self, amount: u8) -> bool {
        if amount >= self.current_health {
            self.current_health = 0;
            return true;
        }

        self.current_health -= amount;
        false
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PlayerId {
    South,
    North,
}

#[derive(Debug)]
pub struct TurnState {
    player_id: PlayerId,
    current_pool: Pool,
    mana_increased: bool,
}

impl TurnState {
    fn new(player_id: PlayerId, current_pool: Pool) -> Self {
        Self {
            player_id,
            current_pool,
            mana_increased: false,
        }
    }

    pub fn current_pool(&self) -> Pool {
        self.current_pool
    }

    pub fn has_increased_mana(&self) -> bool {
        self.mana_increased
    }

    fn refresh_pool(&mut self, pool: Pool) {
        self.current_pool = pool;
    }

    fn try_pay(&mut self, cost: Pool) -> bool {
        if cost.fire > self.current_pool.fire
            || cost.air > self.current_pool.air
            || cost.earth > self.current_pool.earth
            || cost.water > self.current_pool.water
        {
            return false;
        }

        self.current_pool.fire -= cost.fire;
        self.current_pool.air -= cost.air;
        self.current_pool.earth -= cost.earth;
        self.current_pool.water -= cost.water;

        true
    }
}

#[derive(Debug, Clone)]
struct PlayerState {
    max_pool: Pool,
    orb: Orb,
    units: Vec<Unit>,
    roster: Vec<&'static UnitDefinition>,
}

impl Default for PlayerState {
    fn default() -> Self {
        let test_roster: Vec<&'static UnitDefinition> = vec![
            &unit_catalog::MOSSBACK_TORTOISE,
            &unit_catalog::EMBER_FOX,
            &unit_catalog::ROOTGUARD_DRYAD,
            &unit_catalog::ASHBARK_STAG,
            &unit_catalog::VOLCANIC_TREANT,
        ];

        Self {
            max_pool: Pool::default(),
            orb: Orb::default(),
            units: Vec::default(),
            roster: test_roster,
        }
    }
}

impl PlayerState {
    fn increase_mana_pool(&mut self, up: Element) {
        self.max_pool.up(up);
    }

    fn add_unit(&mut self, unit: Unit) {
        self.units.push(unit);
    }

    fn remove_unit(&mut self, unit_id: u16) {
        if let Some(i) = self.units.iter().position(|unit| unit.id == unit_id) {
            self.units.remove(i);
        }
    }
}

#[derive(Debug)]
pub struct GameState {
    pub board: Board,
    player_south: PlayerState,
    player_north: PlayerState,
    next_unit_id: u16,
    current_turn_id: u16,
    turn: TurnState,
}

impl Default for GameState {
    fn default() -> Self {
        let player_south = PlayerState::default();
        let player_north = PlayerState::default();

        let player_id = PlayerId::South;
        let current_pool = player_south.max_pool;

        Self {
            board: Board::default(),
            player_south,
            player_north,
            next_unit_id: 0,
            current_turn_id: 0,
            turn: TurnState::new(player_id, current_pool),
        }
    }
}

impl GameState {
    fn player(&self, player_id: PlayerId) -> &PlayerState {
        match player_id {
            PlayerId::South => &self.player_south,
            PlayerId::North => &self.player_north,
        }
    }

    fn player_mut(&mut self, player_id: PlayerId) -> &mut PlayerState {
        match player_id {
            PlayerId::South => &mut self.player_south,
            PlayerId::North => &mut self.player_north,
        }
    }

    pub fn roster_from_player(&self, player_id: PlayerId) -> &[&'static UnitDefinition] {
        match player_id {
            PlayerId::South => &self.player_south.roster,
            PlayerId::North => &self.player_north.roster,
        }
    }

    fn unit(&self, id: u16) -> Option<(&Unit, PlayerId)> {
        if let Some(unit) = self.player_south.units.iter().find(|unit| unit.id == id) {
            return Some((unit, PlayerId::South));
        }

        if let Some(unit) = self.player_north.units.iter().find(|unit| unit.id == id) {
            return Some((unit, PlayerId::North));
        }

        None
    }

    fn unit_mut(&mut self, id: u16) -> Option<(&mut Unit, PlayerId)> {
        if let Some(unit) = self
            .player_south
            .units
            .iter_mut()
            .find(|unit| unit.id == id)
        {
            return Some((unit, PlayerId::South));
        }

        if let Some(unit) = self
            .player_north
            .units
            .iter_mut()
            .find(|unit| unit.id == id)
        {
            return Some((unit, PlayerId::North));
        }

        None
    }

    pub fn units_from_player(&self, player_id: PlayerId) -> &[Unit] {
        match player_id {
            PlayerId::South => &self.player_south.units,
            PlayerId::North => &self.player_north.units,
        }
    }

    fn is_hex_occupied(&self, position: Hex) -> bool {
        self.player_south
            .units
            .iter()
            .any(|unit| unit.position == position)
            || self
                .player_north
                .units
                .iter()
                .any(|unit| unit.position == position)
    }

    pub fn current_player_id(&self) -> PlayerId {
        match self.current_turn_id % 2 {
            0 => PlayerId::South,
            _ => PlayerId::North,
        }
    }

    pub fn current_player_max_pool(&self) -> Pool {
        let player_id = self.current_player_id();
        let player = self.player(player_id);

        player.max_pool
    }

    pub fn turn(&self) -> &TurnState {
        &self.turn
    }

    pub fn end_turn(&mut self) {
        self.current_turn_id += 1;

        self.start_new_turn();
    }

    fn start_new_turn(&mut self) {
        let player_id = self.current_player_id();

        self.turn = TurnState::new(player_id, self.player(player_id).max_pool)
    }

    pub fn increase_current_player_mana(&mut self, up: Element) -> bool {
        if self.turn.mana_increased {
            return false;
        }

        let player_id = self.current_player_id();

        let new_pool = {
            let player = self.player_mut(player_id);
            player.increase_mana_pool(up);
            player.max_pool
        };

        self.turn.refresh_pool(new_pool);
        self.turn.mana_increased = true;

        true
    }

    pub fn spawn_unit(&mut self, unit_type: &'static UnitDefinition, position: Hex) -> bool {
        let player_id = self.current_player_id();

        if !self.board.is_spawn_hex(position, player_id) || self.is_hex_occupied(position) {
            return false;
        }

        if !self.turn.try_pay(unit_type.cost) {
            return false;
        }

        self.next_unit_id += 1;

        let unit = Unit {
            id: self.next_unit_id,
            kind: unit_type,
            position: position,
            current_attack: unit_type.attack,
            current_health: unit_type.health,
            current_speed: unit_type.speed,
        };

        let player = self.player_mut(player_id);
        player.add_unit(unit);

        true
    }

    fn unit_combat(&mut self, attacker_id: u16, target_id: u16) {
        let damage = self.unit(attacker_id).unwrap().0.current_attack;

        let (unit, player_id) = self.unit_mut(target_id).unwrap();

        let is_dead = unit.take_damage(damage);

        if is_dead {
            let player = self.player_mut(player_id);
            player.remove_unit(target_id);
        }
    }
}

#[cfg(test)]
mod tests;
