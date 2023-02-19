use std::{io::stdin, str::SplitWhitespace, time::Duration};

use chess::Board;

use crate::engine::game::Game;

use super::time_management::TimeManagement;

pub struct Cli {
    game: Game,
}

impl Cli {
    pub fn new() -> Cli {
        Cli {
            game: Default::default(),
        }
    }

    pub fn execute(&mut self) {
        loop {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let mut words = input.trim().split_whitespace();
            let option = words.next();
            match option {
                Some(_) => {}
                None => continue,
            }
            let command = option.unwrap();
            let args = words;
            match command {
                "uci" => {
                    self.uci();
                }

                "isReady" => {
                    self.is_ready();
                }

                "position" => {
                    self.position();
                }

                "go" => {
                    self.go(args);
                }

                "quit" => return,
                _ => continue,
            }
        }
    }

    fn is_ready(&self) {
        self.send_ready_ok();
    }

    fn position(&self) {
        //TODO
    }

    fn go(&mut self, mut args: SplitWhitespace) {
        let mut tm: TimeManagement = Default::default();
        loop {
            match args.next() {
                Some(cmd) => match cmd {
                    "searchmoves" => {}

                    "ponder" => {}

                    "wtime" => match args.next() {
                        Some(arg) => match arg.parse() {
                            Ok(a) => tm.white_time = a,
                            Err(_) => break,
                        },
                        None => break,
                    },

                    "btime" => match args.next() {
                        Some(arg) => match arg.parse() {
                            Ok(a) => tm.black_time = a,
                            Err(_) => break,
                        },
                        None => break,
                    },

                    "winc" => match args.next() {
                        Some(arg) => match arg.parse() {
                            Ok(a) => tm.white_inc = a,
                            Err(_) => break,
                        },
                        None => break,
                    },

                    "binc" => match args.next() {
                        Some(arg) => match arg.parse() {
                            Ok(a) => tm.black_inc = a,
                            Err(_) => break,
                        },
                        None => break,
                    },

                    "movestogo" => match args.next() {
                        Some(arg) => match arg.parse() {
                            Ok(a) => tm.moves_to_go = a,
                            Err(_) => break,
                        },
                        None => break,
                    },

                    "depth" => match args.next() {
                        Some(arg) => match arg.parse() {
                            Ok(a) => self.game.max_depth = a,
                            Err(_) => break,
                        },
                        None => break,
                    },

                    "nodes" => {}

                    "mate" => {}

                    "movetime" => match args.next() {
                        Some(arg) => match arg.parse::<u16>() {
                            Ok(a) => {
                                self.game.move_time = Duration::from_millis(u64::from(a * 9 / 10));
                                self.timer_start();
                            }
                            Err(_) => break,
                        },
                        None => break,
                    },

                    _ => break,
                },
                None => break,
            }
        }
    }

    fn timer_start(&mut self) {
        // TODO Start a timer
        // TODO create a channel
        self.game.playing.set(true);
        match self.game.find_move() {
            Some(m) => {
                //let result: &mut Board = self.
                //let _ = self.game.board.make_move(m, result);
            }
            None => panic!("No valid move found"),
        }
    }

    fn uci(&self) {
        self.send_id();
        self.send_options();
        self.send_uci_ok();
    }

    fn send_id(&self) {
        println!("id name C4E5R");
        println!("id author Eugen Lindorfer");
    }

    fn send_options(&self) {
        println!("option"); //TODO extend this
    }

    fn send_uci_ok(&self) {
        println!("uciok");
    }

    fn send_ready_ok(&self) {
        println!("readyok");
    }
}
