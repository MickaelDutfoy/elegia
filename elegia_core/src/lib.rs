pub mod unit_catalog;

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
struct UnitDefinition {
    name: &'static str,
    cost: Pool,
    attack: u8,
    health: u8,
    speed: u8,
}

#[derive(Debug)]
struct Unit {
    name: &'static str,
    cost: Pool,
    current_attack: u8,
    current_health: u8,
    current_speed: u8,
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

    fn spawn_unit(&mut self, unit_type: &UnitDefinition) -> bool {
        if !self.try_pay(unit_type.cost) {
            return false;
        }

        let unit = Unit {
            name: unit_type.name,
            cost: unit_type.cost,
            current_attack: unit_type.attack,
            current_health: unit_type.health,
            current_speed: unit_type.speed,
        };

        self.units.push(unit);
        true
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
        let mut player = PlayerState::default();

        let tortoise = unit_catalog::find_unit_by_name("Mossback Tortoise").unwrap();

        let result = player.spawn_unit(tortoise);

        assert_eq!(result, false);
        assert_eq!(player.units.len(), 0);
    }

    #[test]
    fn can_spawn_an_unit_from_catalog() {
        let mut player = PlayerState::default();

        player.start_turn(Element::Earth);
        player.start_turn(Element::Earth);

        let tortoise = unit_catalog::find_unit_by_name("Mossback Tortoise").unwrap();

        let result = player.spawn_unit(tortoise);

        assert_eq!(result, true);
        assert_eq!(player.units.len(), 1);
    }
}
