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

#[derive(Debug)]
struct Unit {
    id: u16,
    name: &'static str,
    cost: Pool,
    current_attack: u8,
    current_health: u8,
    current_speed: u8,
}

#[derive(Debug)]
enum PlayerId {
    One,
    Two,
}

#[derive(Debug, Default)]
struct GameState {
    player_one: PlayerState,
    player_two: PlayerState,
    next_unit_id: u16,
}

impl GameState {
    fn get_player_mut(&mut self, player_id: &PlayerId) -> &mut PlayerState {
        match player_id {
            PlayerId::One => &mut self.player_one,
            PlayerId::Two => &mut self.player_two,
        }
    }

    fn spawn_unit(&mut self, player_id: PlayerId, unit_type: &UnitDefinition) -> bool {
        {
            let player = self.get_player_mut(&player_id);

            if !player.try_pay(unit_type.cost) {
                return false;
            }
        }

        self.next_unit_id += 1;

        let unit = Unit {
            id: self.next_unit_id,
            name: unit_type.name,
            cost: unit_type.cost,
            current_attack: unit_type.attack,
            current_health: unit_type.health,
            current_speed: unit_type.speed,
        };

        let player = self.get_player_mut(&player_id);
        player.units.push(unit);

        true
    }
}

#[derive(Debug, Default)]
struct PlayerState {
    max_pool: Pool,
    current_pool: Pool,
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

    fn find_unit_by_id(&self, id: u16) -> Option<&Unit> {
        self.units.iter().find(|u| u.id == id)
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

        let result = game.spawn_unit(PlayerId::One, tortoise);

        assert_eq!(result, false);
        assert_eq!(game.player_one.units.len(), 0);
    }

    #[test]
    fn can_spawn_a_unit_from_catalog() {
        let mut game = GameState::default();

        game.player_one.start_turn(Element::Earth);
        game.player_one.start_turn(Element::Earth);

        let tortoise = unit_catalog::find_unit_by_name("Mossback Tortoise").unwrap();

        let result = game.spawn_unit(PlayerId::One, tortoise);

        assert_eq!(result, true);
        assert_eq!(game.player_one.units.len(), 1);
    }

    #[test]
    fn unit_ids_increment_as_intended() {
        let mut game = GameState::default();

        game.player_one.start_turn(Element::Earth);
        game.player_one.start_turn(Element::Earth);

        game.player_two.start_turn(Element::Fire);
        game.player_two.start_turn(Element::Fire);

        let tortoise = unit_catalog::find_unit_by_name("Mossback Tortoise").unwrap();
        let fox = unit_catalog::find_unit_by_name("Ember Fox").unwrap();

        let result1 = game.spawn_unit(PlayerId::One, tortoise);
        let result2 = game.spawn_unit(PlayerId::Two, tortoise);
        let result3 = game.spawn_unit(PlayerId::Two, fox);

        assert_eq!(result1, true);
        assert_eq!(result2, false);
        assert_eq!(result3, true);

        assert_eq!(game.player_one.units.len(), 1);
        assert_eq!(game.player_two.units.len(), 1);

        assert_eq!(game.player_one.units[0].id, 1);
        assert_eq!(game.player_two.units[0].id, 2);
    }
}
