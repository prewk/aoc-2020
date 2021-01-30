use crate::mem_game::Game;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Game {
    Game::from(input)
}

#[aoc(day15, part1)]
pub fn part1(imm_game: &Game) -> Option<usize> {
    let mut game = imm_game.clone();

    for _ in 0..(2020 - 1) {
        game.turn();
    }

    game.turn()
}
