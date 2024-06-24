mod bluetooth_server;
mod golf_game;

use bluetooth_server::run_bluetooth_server;
use std::{
    fs::OpenOptions,
    io::Write,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Instant,
};

use crate::golf_game::{GameCommand, GameState};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (tx, rx): (Sender<GameCommand>, Receiver<GameCommand>) = mpsc::channel();
    thread::spawn(|| game_loop(rx));
    _ = run_bluetooth_server(tx).await;

    Ok(())
}

fn game_loop(rx: Receiver<GameCommand>) {
    let mut game_state = GameState::new();
    let mut now = Instant::now();
    loop {
        let delta = Instant::now().duration_since(now).as_secs_f32();
        now = Instant::now();
        // advance the game by a frame
        let game_command = rx.try_recv().unwrap_or(GameCommand::None);
        game_state.calc_next_state(game_command, delta);

        // write updadated game to file
        let my_game_serialized = serde_json::to_string(&game_state).unwrap().into_bytes();
        let mut golf_file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open("golf_info.json")
            .unwrap();
        golf_file.write(&my_game_serialized).unwrap();
    }
}
