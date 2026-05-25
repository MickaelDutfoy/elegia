use super::*;

#[test]
fn game_starts_with_south_player() {
    let game = GameState::default();

    assert_eq!(game.turn.player_id, PlayerId::South)
}

#[test]
fn ending_turn_changes_active_player() {
    let mut game = GameState::default();
    game.end_turn();

    assert_eq!(game.turn.player_id, PlayerId::North)
}

#[test]
fn up_max_mana_refreshes_current() {
    let mut game = GameState::default();

    assert_eq!(game.turn.mana_increased, false);

    game.increase_current_player_mana(Element::Fire);

    assert_eq!(
        game.player_south.max_pool,
        Pool {
            fire: 1,
            ..Pool::default()
        }
    );

    assert_eq!(
        game.turn.current_pool,
        Pool {
            fire: 1,
            ..Pool::default()
        }
    );

    assert_eq!(game.turn.mana_increased, true);
}

#[test]
fn cant_up_mana_twice_a_turn() {
    let mut game = GameState::default();

    let first_try = game.increase_current_player_mana(Element::Fire);
    let second_try = game.increase_current_player_mana(Element::Air);

    assert_eq!(first_try, true);
    assert_eq!(second_try, false);

    assert_eq!(
        game.turn.current_pool,
        Pool {
            fire: 1,
            ..Pool::default()
        }
    );
}

#[test]
fn cant_pay_mana_cost() {
    let mut game = GameState::default();

    let cost = Pool {
        water: 2,
        ..Pool::default()
    };

    let result = game.turn.try_pay(cost);

    assert_eq!(result, false);
}

#[test]
fn new_turn_refreshes_mana() {
    let mut game = GameState::default();

    game.increase_current_player_mana(Element::Fire);

    let cost = Pool {
        fire: 1,
        ..Pool::default()
    };

    let result = game.turn.try_pay(cost);

    assert_eq!(result, true);

    assert_eq!(game.turn.current_pool, Pool::default());

    game.end_turn();
    game.end_turn();

    assert_eq!(
        game.turn.current_pool,
        Pool {
            fire: 1,
            ..Pool::default()
        }
    );
}

#[test]
fn successful_payment_decreases_multiple_elements() {
    let mut game = GameState::default();

    game.increase_current_player_mana(Element::Water);
    game.end_turn();
    game.end_turn();
    game.increase_current_player_mana(Element::Water);
    game.end_turn();
    game.end_turn();
    game.increase_current_player_mana(Element::Earth);

    let cost = Pool {
        water: 1,
        earth: 1,
        ..Pool::default()
    };

    let result = game.turn.try_pay(cost);

    assert_eq!(result, true);

    assert_eq!(
        game.turn.current_pool,
        Pool {
            water: 1,
            ..Pool::default()
        }
    );
}

#[test]
fn failed_spawn_attempt_if_cant_afford() {
    let mut game = GameState::default();

    let tortoise = &unit_catalog::MOSSBACK_TORTOISE;

    let result = game.spawn_unit(tortoise, Hex { q: 0, r: -3 });

    assert_eq!(result, false);
    assert_eq!(game.player_south.units.len(), 0);
}

#[test]
fn can_spawn_a_unit_from_catalog() {
    let mut game = GameState::default();

    let tortoise = &unit_catalog::MOSSBACK_TORTOISE;

    game.increase_current_player_mana(Element::Earth);
    game.end_turn();
    game.end_turn();
    game.increase_current_player_mana(Element::Earth);

    let result = game.spawn_unit(tortoise, Hex { q: 0, r: 3 });

    assert_eq!(result, true);
    assert_eq!(game.player_south.units.len(), 1);
}

#[test]
fn unit_ids_increment_as_intended() {
    let mut game = GameState::default();

    let tortoise = &unit_catalog::MOSSBACK_TORTOISE;
    let fox = &unit_catalog::EMBER_FOX;

    game.increase_current_player_mana(Element::Earth);
    game.end_turn();
    game.increase_current_player_mana(Element::Fire);
    game.end_turn();
    game.increase_current_player_mana(Element::Earth);

    let first_result = game.spawn_unit(tortoise, Hex { q: 0, r: 3 });

    game.end_turn();
    game.increase_current_player_mana(Element::Fire);

    let second_result = game.spawn_unit(tortoise, Hex { q: 0, r: -3 });
    let third_result = game.spawn_unit(fox, Hex { q: 0, r: -3 });

    assert_eq!(first_result, true);
    assert_eq!(second_result, false);
    assert_eq!(third_result, true);

    assert_eq!(game.player_south.units[0].id, 1);
    assert_eq!(game.player_north.units[0].id, 2);
}

#[test]
fn can_get_an_units_details_from_id() {
    let mut game = GameState::default();

    let tortoise = &unit_catalog::MOSSBACK_TORTOISE;

    game.increase_current_player_mana(Element::Earth);
    game.end_turn();
    game.end_turn();
    game.increase_current_player_mana(Element::Earth);

    game.spawn_unit(tortoise, Hex { q: 0, r: 3 });

    let summoned_tortoise = game.unit_mut(1).unwrap();

    assert_eq!(summoned_tortoise.1, PlayerId::South);
    assert_eq!(summoned_tortoise.0.current_health, 3);
}

#[test]
fn combat_applies_damage_to_target() {
    let mut game = GameState::default();

    let tortoise = &unit_catalog::MOSSBACK_TORTOISE;
    let falcon = &unit_catalog::ZEPHYR_FALCON;

    game.increase_current_player_mana(Element::Earth);
    game.end_turn();
    game.increase_current_player_mana(Element::Air);
    game.end_turn();
    game.increase_current_player_mana(Element::Earth);

    game.spawn_unit(tortoise, Hex { q: 0, r: 3 });

    game.end_turn();
    game.increase_current_player_mana(Element::Air);

    game.spawn_unit(falcon, Hex { q: 0, r: -3 });

    let tortoise_id = game.player_south.units[0].id;
    let falcon_id = game.player_north.units[0].id;

    game.unit_combat(falcon_id, tortoise_id);

    let tortoise = game.unit(tortoise_id).unwrap().0;

    assert_eq!(tortoise.current_health, 2);
}

#[test]
fn dead_unit_is_removed_from_players_vec() {
    let mut game = GameState::default();

    let falcon = &unit_catalog::ZEPHYR_FALCON;
    let fox = &unit_catalog::EMBER_FOX;

    game.increase_current_player_mana(Element::Air);
    game.end_turn();
    game.increase_current_player_mana(Element::Fire);
    game.end_turn();
    game.increase_current_player_mana(Element::Air);

    game.spawn_unit(falcon, Hex { q: 0, r: 3 });

    game.end_turn();
    game.increase_current_player_mana(Element::Fire);

    game.spawn_unit(fox, Hex { q: 0, r: -3 });

    game.unit_combat(2, 1);

    assert_eq!(game.player_south.units.len(), 0);
    assert_eq!(game.player_north.units.len(), 1);
}

#[test]
fn cant_summon_outside_spawn_zone_or_orb() {
    let mut game = GameState::default();

    let tortoise = &unit_catalog::MOSSBACK_TORTOISE;

    game.increase_current_player_mana(Element::Earth);
    game.end_turn();
    game.end_turn();
    game.increase_current_player_mana(Element::Earth);

    let first_result = game.spawn_unit(tortoise, Hex { q: 2, r: 3 });

    assert_eq!(first_result, false);

    assert_eq!(
        game.turn.current_pool,
        Pool {
            earth: 2,
            ..Pool::default()
        }
    );

    let second_result = game.spawn_unit(tortoise, Hex { q: 2, r: 4 });

    assert_eq!(second_result, false);
}

#[test]
fn cant_summon_an_unit_on_occupied_hex() {
    let mut game = GameState::default();

    let tortoise = &unit_catalog::MOSSBACK_TORTOISE;
    let fox = &unit_catalog::EMBER_FOX;

    game.increase_current_player_mana(Element::Earth);
    game.end_turn();
    game.end_turn();
    game.increase_current_player_mana(Element::Earth);

    let first_result = game.spawn_unit(tortoise, Hex { q: 0, r: 3 });

    game.end_turn();
    game.end_turn();
    game.increase_current_player_mana(Element::Fire);
    game.end_turn();
    game.end_turn();
    game.increase_current_player_mana(Element::Fire);

    let second_result = game.spawn_unit(fox, Hex { q: 0, r: 3 });

    assert_eq!(first_result, true);
    assert_eq!(second_result, false);
}
