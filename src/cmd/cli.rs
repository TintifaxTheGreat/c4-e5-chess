use super::time_management::TimeManagement;
use crate::engine::game::Game;
use chess::Board;
use log::info;
use std::{
    io::stdin,
    mem,
    str::{FromStr, SplitWhitespace},
    sync::{atomic::Ordering, mpsc::channel},
};
use timer::Timer;

pub struct Cli {
    game: Game,
    tm: TimeManagement,
}

impl Cli {
    pub fn new() -> Cli {
        Cli {
            game: Default::default(),
            tm: TimeManagement::default(),
        }
    }

    pub fn execute(&mut self) {
        loop {
            info!("im in loop");
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

                "isready" => {
                    self.is_ready();
                }

                "position" => {
                    self.position(args);
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

    fn position(&mut self, mut args: SplitWhitespace) {
        loop {
            match args.next() {
                Some(cmd) => match cmd {
                    "fen" => match args.next() {
                        Some(arg) => match arg {
                            pos => match Board::from_str(pos) {
                                Ok(b) => self.game.board = b,
                                Err(_) => panic!("FEN not valid"),
                            },
                        },

                        None => break,
                    },

                    // do nothing as game was already initialised with startposition
                    "startpos" => {}

                    _ => break,
                },
                None => break,
            }
        }
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
                                self.game.move_time = a * 9 / 10;
                                self.timer_start();
                                return;
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
        self.tm.set_game_time(&mut self.game);
        self.timer_start();

    }

    fn timer_start(&mut self) {
        let timer = Timer::new();
        let (tx, rx) = channel();

        self.game.playing.store(true, Ordering::Relaxed);
        let stop_bool = self.game.playing.clone();
        _ = timer.schedule_with_delay(
            chrono::Duration::milliseconds(self.game.move_time.clone() as i64),
            move || {
                stop_bool.store(true, Ordering::Relaxed);
                let _ignored = tx.send(());
            },
        );

        match self.game.find_move() {
            Some(m) => {
                let mut bresult = mem::MaybeUninit::<Board>::uninit();

                unsafe {
                    let _ = &self.game.board.make_move(m, &mut *bresult.as_mut_ptr());
                }
                //info!("im there");
                //println!("bestmove %", m.to_string()); //TODO contunie herer
            }
            None => panic!("No valid move found"),
        }

        rx.recv().unwrap();
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
