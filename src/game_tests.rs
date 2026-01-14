use crate::game::{Game, Player};

fn place_black_wins(mut game: Game, moves: &[(usize, usize)]) -> Game {
    // Interleave white moves off the main line so black can still place consecutively.
    for (i, (r, c)) in moves.iter().copied().enumerate() {
        game.place(r, c).unwrap();
        if i + 1 != moves.len() {
            game.place(0, i).unwrap();
        }
    }
    game
}

#[test]
fn horizontal_five_is_win() {
    let game = Game::new(15);
    let moves = [(7, 3), (7, 4), (7, 5), (7, 6), (7, 7)];
    let game = place_black_wins(game, &moves);
    assert_eq!(game.winner(), Some(Player::Black));
}

#[test]
fn vertical_five_is_win() {
    let game = Game::new(15);
    let moves = [(3, 7), (4, 7), (5, 7), (6, 7), (7, 7)];
    let game = place_black_wins(game, &moves);
    assert_eq!(game.winner(), Some(Player::Black));
}

#[test]
fn diagonal_down_right_five_is_win() {
    let game = Game::new(15);
    let moves = [(3, 3), (4, 4), (5, 5), (6, 6), (7, 7)];
    let game = place_black_wins(game, &moves);
    assert_eq!(game.winner(), Some(Player::Black));
}

#[test]
fn diagonal_down_left_five_is_win() {
    let game = Game::new(15);
    let moves = [(3, 11), (4, 10), (5, 9), (6, 8), (7, 7)];
    let game = place_black_wins(game, &moves);
    assert_eq!(game.winner(), Some(Player::Black));
}
