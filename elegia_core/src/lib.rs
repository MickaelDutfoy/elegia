pub mod unit_catalog;
use crate::unit_catalog::UnitDefinition;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Element {
    Fire,
    Water,
    Air,
    Earth,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Pool {
    fire: u8,
    water: u8,
    air: u8,
    earth: u8,
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

#[derive(Debug)]
pub struct Board {
    radius: u8,
}

impl Default for Board {
    fn default() -> Self {
        Self { radius: 4 }
    }
}

impl Board {
    fn is_valid_hex(&self, hex: Hex) -> bool {
        let q = hex.q;
        let r = hex.r;
        let s = -q - r;

        q.abs().max(r.abs()).max(s.abs()) <= self.radius.try_into().unwrap()
    }

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
struct Unit {
    id: u16,
    kind: &'static UnitDefinition,
    position: Hex,
    current_attack: u8,
    current_health: u8,
    current_speed: u8,
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

#[derive(Debug)]
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

#[derive(Debug, Default)]
struct PlayerState {
    max_pool: Pool,
    current_pool: Pool,
    orb: Orb,
    units: Vec<Unit>,
}

impl PlayerState {
    fn start_turn(&mut self, up: Element) {
        self.max_pool.up(up);
        self.current_pool = self.max_pool;
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

    fn add_unit(&mut self, unit: Unit) {
        self.units.push(unit);
    }

    fn remove_unit(&mut self, unit_id: u16) {
        if let Some(i) = self.units.iter().position(|unit| unit.id == unit_id) {
            self.units.remove(i);
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PlayerId {
    South,
    North,
}

#[derive(Debug, Default)]
struct GameState {
    board: Board,
    player_south: PlayerState,
    player_north: PlayerState,
    next_unit_id: u16,
}

impl GameState {
    fn get_player_mut(&mut self, player_id: PlayerId) -> &mut PlayerState {
        match player_id {
            PlayerId::South => &mut self.player_south,
            PlayerId::North => &mut self.player_north,
        }
    }

    fn get_unit_by_id(&self, id: u16) -> Option<(&Unit, PlayerId)> {
        if let Some(unit) = self.player_south.units.iter().find(|unit| unit.id == id) {
            return Some((unit, PlayerId::South));
        }

        if let Some(unit) = self.player_north.units.iter().find(|unit| unit.id == id) {
            return Some((unit, PlayerId::North));
        }

        None
    }

    fn get_unit_by_id_mut(&mut self, id: u16) -> Option<(&mut Unit, PlayerId)> {
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

    fn spawn_unit(
        &mut self,
        player_id: PlayerId,
        unit_type: &'static UnitDefinition,
        position: Hex,
    ) -> bool {
        if !self.board.is_spawn_hex(position, player_id) || self.is_hex_occupied(position) {
            return false;
        }

        {
            let player = self.get_player_mut(player_id);

            if !player.try_pay(unit_type.cost) {
                return false;
            }
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

        let player = self.get_player_mut(player_id);
        player.add_unit(unit);

        true
    }

    fn unit_combat(&mut self, attacker_id: u16, target_id: u16) {
        let damage = self.get_unit_by_id(attacker_id).unwrap().0.current_attack;

        let (unit, player_id) = self.get_unit_by_id_mut(target_id).unwrap();

        let is_dead = unit.take_damage(damage);

        if is_dead {
            let player = self.get_player_mut(player_id);
            player.remove_unit(target_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turn_1_up_fire_refreshes_current() {
        let mut player = PlayerState::default();

        player.start_turn(Element::Fire);

        assert_eq!(
            player.max_pool,
            Pool {
                fire: 1,
                ..Pool::default()
            }
        );
        assert_eq!(
            player.current_pool,
            Pool {
                fire: 1,
                ..Pool::default()
            }
        );
    }

    #[test]
    fn turn_2_up_water_accumulates_in_max_and_current() {
        let mut player = PlayerState::default();

        player.start_turn(Element::Fire);
        player.start_turn(Element::Water);

        assert_eq!(
            player.max_pool,
            Pool {
                fire: 1,
                water: 1,
                ..Pool::default()
            }
        );
        assert_eq!(
            player.current_pool,
            Pool {
                fire: 1,
                water: 1,
                ..Pool::default()
            }
        );
    }

    #[test]
    fn refresh_overwrites_spent_current_pool() {
        let mut player = PlayerState::default();

        player.start_turn(Element::Fire);
        player.current_pool.fire = 0; // simulate consuming fire
        player.start_turn(Element::Air);

        assert_eq!(
            player.max_pool,
            Pool {
                fire: 1,
                air: 1,
                ..Pool::default()
            }
        );
        assert_eq!(
            player.current_pool,
            Pool {
                fire: 1,
                air: 1,
                ..Pool::default()
            }
        );
    }

    #[test]
    fn cant_pay_mana_cost() {
        let mut player = PlayerState::default();

        player.start_turn(Element::Water);

        let cost = Pool {
            water: 2,
            ..Pool::default()
        };

        let result = player.try_pay(cost);

        assert_eq!(result, false);

        assert_eq!(
            player.current_pool,
            Pool {
                water: 1,
                ..Pool::default()
            }
        );
    }

    #[test]
    fn successful_payment_decreases_multiple_elements() {
        let mut player = PlayerState::default();

        player.start_turn(Element::Water);
        player.start_turn(Element::Water);
        player.start_turn(Element::Earth);

        let cost = Pool {
            water: 1,
            earth: 1,
            ..Pool::default()
        };

        let result = player.try_pay(cost);

        assert_eq!(result, true);

        assert_eq!(
            player.current_pool,
            Pool {
                water: 1,
                ..Pool::default()
            }
        );
    }

    #[test]
    fn failed_spawn_attempt_if_cant_afford() {
        let mut game = GameState::default();

        let tortoise = unit_catalog::find_unit_by_name("Mossback Tortoise").unwrap();

        let result = game.spawn_unit(PlayerId::South, tortoise, Hex { q: 0, r: -3 });

        assert_eq!(result, false);
        assert_eq!(game.player_south.units.len(), 0);
    }

    #[test]
    fn can_spawn_a_unit_from_catalog() {
        let mut game = GameState::default();

        game.player_south.start_turn(Element::Earth);
        game.player_south.start_turn(Element::Earth);

        let tortoise = unit_catalog::find_unit_by_name("Mossback Tortoise").unwrap();

        let result = game.spawn_unit(PlayerId::South, tortoise, Hex { q: 0, r: -3 });

        assert_eq!(result, true);
        assert_eq!(game.player_south.units.len(), 1);
    }

    #[test]
    fn unit_ids_increment_as_intended() {
        let mut game = GameState::default();

        game.player_south.start_turn(Element::Earth);
        game.player_south.start_turn(Element::Earth);

        game.player_north.start_turn(Element::Fire);
        game.player_north.start_turn(Element::Fire);

        let tortoise = unit_catalog::find_unit_by_name("Mossback Tortoise").unwrap();
        let fox = unit_catalog::find_unit_by_name("Ember Fox").unwrap();

        let result1 = game.spawn_unit(PlayerId::South, tortoise, Hex { q: 0, r: -3 });
        let result2 = game.spawn_unit(PlayerId::North, tortoise, Hex { q: 0, r: 3 });
        let result3 = game.spawn_unit(PlayerId::North, fox, Hex { q: 0, r: 3 });

        assert_eq!(result1, true);
        assert_eq!(result2, false);
        assert_eq!(result3, true);

        assert_eq!(game.player_south.units[0].id, 1);
        assert_eq!(game.player_north.units[0].id, 2);
    }

    #[test]
    fn can_get_an_units_details_from_id() {
        let mut game = GameState::default();

        game.player_north.start_turn(Element::Earth);
        game.player_north.start_turn(Element::Earth);

        let tortoise = unit_catalog::find_unit_by_name("Mossback Tortoise").unwrap();

        game.spawn_unit(PlayerId::North, tortoise, Hex { q: 0, r: 3 });

        let summoned_tortoise = game.get_unit_by_id_mut(1).unwrap();

        assert_eq!(summoned_tortoise.1, PlayerId::North);
        assert_eq!(summoned_tortoise.0.current_health, 5);
    }

    #[test]
    fn combat_applies_damage_to_target() {
        let mut game = GameState::default();

        game.player_south.start_turn(Element::Earth);
        game.player_south.start_turn(Element::Earth);

        game.player_north.start_turn(Element::Fire);
        game.player_north.start_turn(Element::Fire);

        let tortoise = unit_catalog::find_unit_by_name("Mossback Tortoise").unwrap();
        let fox = unit_catalog::find_unit_by_name("Ember Fox").unwrap();

        game.spawn_unit(PlayerId::South, tortoise, Hex { q: 0, r: -3 });
        game.spawn_unit(PlayerId::North, fox, Hex { q: 0, r: 3 });

        let tortoise_id = game.player_south.units[0].id;
        let fox_id = game.player_north.units[0].id;

        game.unit_combat(fox_id, tortoise_id);

        let tortoise = game.get_unit_by_id(tortoise_id).unwrap().0;

        assert_eq!(tortoise.current_health, 2);
    }

    #[test]
    fn dead_unit_is_removed_from_players_vec() {
        let mut game = GameState::default();

        game.player_south.start_turn(Element::Air);
        game.player_south.start_turn(Element::Air);

        game.player_north.start_turn(Element::Fire);
        game.player_north.start_turn(Element::Fire);

        let falcon = unit_catalog::find_unit_by_name("Zephyr Falcon").unwrap();
        let fox = unit_catalog::find_unit_by_name("Ember Fox").unwrap();

        game.spawn_unit(PlayerId::South, falcon, Hex { q: 0, r: -3 });
        game.spawn_unit(PlayerId::North, fox, Hex { q: 0, r: 3 });

        game.unit_combat(2, 1);

        assert_eq!(game.player_south.units.len(), 0);
        assert_eq!(game.player_north.units.len(), 1);
    }

    #[test]
    fn cant_summon_an_unit_on_occupied_hex() {
        let mut game = GameState::default();

        game.player_south.start_turn(Element::Earth);
        game.player_south.start_turn(Element::Earth);
        game.player_south.start_turn(Element::Fire);
        game.player_south.start_turn(Element::Fire);

        let tortoise = unit_catalog::find_unit_by_name("Mossback Tortoise").unwrap();
        let fox = unit_catalog::find_unit_by_name("Ember Fox").unwrap();

        let result1 = game.spawn_unit(PlayerId::South, tortoise, Hex { q: 0, r: -3 });
        let result2 = game.spawn_unit(PlayerId::South, fox, Hex { q: 0, r: -3 });

        assert_eq!(result1, true);
        assert_eq!(result2, false);
    }

    #[test]
    fn cant_summon_outside_spawn_zone() {
        let mut game = GameState::default();

        game.player_south.start_turn(Element::Earth);
        game.player_south.start_turn(Element::Earth);

        let tortoise = unit_catalog::find_unit_by_name("Mossback Tortoise").unwrap();

        let result = game.spawn_unit(PlayerId::South, tortoise, Hex { q: 2, r: 3 });

        assert_eq!(result, false);
    }

    fn cant_summon_on_orb() {
        let mut game = GameState::default();

        game.player_south.start_turn(Element::Earth);
        game.player_south.start_turn(Element::Earth);

        let tortoise = unit_catalog::find_unit_by_name("Mossback Tortoise").unwrap();

        let result = game.spawn_unit(PlayerId::South, tortoise, Hex { q: 0, r: -4 });

        assert_eq!(result, false);
    }
}
