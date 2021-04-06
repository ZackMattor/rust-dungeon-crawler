extern crate termion;

use std::io::{stdin, stdout, Write};
use std::sync::mpsc;
use std::{thread, time};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub enum Command {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Exit,
    Unknown,
}

pub struct Controls {}

impl Controls {
    pub fn start() -> mpsc::Receiver<Command> {
        let (sender, receiver) = mpsc::channel();

        thread::spawn(move || {
            let mut controls: Controls = Controls {};
            controls.input_loop(sender);
        });

        return receiver;
    }

    fn input_loop(&self, sender: mpsc::Sender<Command>) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let stdin = stdin();

        // Start the input loop
        for c in stdin.keys() {
            let command = match c.unwrap() {
                Key::Char('q') => Command::Exit,
                Key::Char('w') => Command::MoveUp,
                Key::Char('a') => Command::MoveLeft,
                Key::Char('s') => Command::MoveDown,
                Key::Char('d') => Command::MoveRight,
                _ => Command::Unknown,
            };

            sender.send(command).unwrap();
        }
    }
}
