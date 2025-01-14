use crate::{GameManager, Player, Revolver};

#[test]
fn switch_once() {
    let mut player1 = Player::new(String::from("player1"), 3);
    let mut player2 = Player::new(String::from("player2"), 3);
    let mut game_manager = GameManager::new();
    game_manager.add_player(&mut player1);
    game_manager.add_player(&mut player2);

    game_manager.switch();
    assert_eq!(game_manager.current_player_index(), 1);
}

#[test]
fn switch_twice() {
    let mut player1 = Player::new(String::from("player1"), 3);
    let mut player2 = Player::new(String::from("player2"), 3);
    let mut game_manager = GameManager::new();
    game_manager.add_player(&mut player1);
    game_manager.add_player(&mut player2);

    game_manager.switch();
    game_manager.switch();
    assert_eq!(game_manager.current_player_index(), 0);
}

#[test]
fn shot_all() {
    let mut player1 = Player::new(String::from("player1"), 3);
    let mut revolver = Revolver::new(3);
    revolver.reload();

    for i in 0..=6 {
        revolver.shot(i, &mut player1);
    }
    assert_eq!(player1.health(), 0);
}

#[test]
fn reload() {
    let mut revolver = Revolver::new(7);
    revolver.reload();

    let mut num_live_rounds = 0;
    for item in revolver.cylinder.into_iter() {
        if item == true {
            num_live_rounds += 1;
        }
    }
    assert_eq!(revolver.has_rounds, [true; 7]);
    assert_eq!(num_live_rounds, 7);
}
