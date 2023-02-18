use std::thread;
use std::time::Duration;

use crate::game::Game;

mod capture;
mod game;
mod images;
mod mouse;

fn main() {
    println!("3");
    thread::sleep(Duration::from_secs(1));
    println!("2");
    thread::sleep(Duration::from_secs(1));
    println!("1");
    thread::sleep(Duration::from_secs(1));
    println!("Go");

    let mut game = Game::new();

    loop {
        game.update_field();

        game.choose_field();

        game.update_img();

        if game.has_lost() {
            game.reset();
            game.update_img();
        }
    }
}
