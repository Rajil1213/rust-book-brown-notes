mod guessing_game_v2;
mod recoverable;
mod unrecoverable;

fn main() {
    guessing_game_v2::start();
    recoverable::test();
    unrecoverable::test();
}
