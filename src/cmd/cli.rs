use super::time_management::TimeManagement;
use crate::engine::game::Game;
use chess::{Board, ChessMove};
use log::{error, info};
use std::{
    io::stdin,
    mem,
    str::{FromStr, SplitWhitespace},
};

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
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let mut input_bak = input.clone();
            input_bak.pop();
            let input_bak_str = input_bak.as_str();
            let mut words = input.trim().split_whitespace();
            let option = words.next();

            match option {
                Some(_) => {}
                None => continue,
            }
            let command = option.unwrap();
            let args = words;
            info!("| {}", input_bak_str);
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
                    "fen" => {
                        let mut fen: String = "".to_string();
                        for _ in 1..7 {
                            match args.next() {
                                Some(s) => {
                                    fen = fen + s + " ";
                                }
                                None => {
                                    error!("No FEN found");
                                    return;
                                }
                            }
                        }
                        match Board::from_str(fen.as_str()) {
                            Ok(b) => self.game.board = b,
                            Err(_) => {
                                error!("FEN not valid");
                                return;
                            }
                        }
                    }

                    // do nothing as game was already initialised with startposition
                    "startpos" => {}

                    "moves" => loop {
                        match args.next() {
                            Some(move_string) => {
                                let mut result = Board::default();
                                match ChessMove::from_str(move_string) {
                                    Ok(m) => {
                                        self.game.board.make_move(m, &mut result);
                                        self.game.board = result;
                                    }
                                    Err(_) => {
                                        error!("Illegal move");
                                        return;
                                    }
                                }
                            }
                            None => return,
                        }
                    },

                    _ => break,
                },
                None => break,
            }
        }
    }

    fn go(&mut self, mut args: SplitWhitespace) {
        loop {
            match args.next() {
                Some(cmd) => match cmd {
                    "searchmoves" => {}

                    "ponder" => {}

                    "wtime" => match args.next() {
                        Some(arg) => match arg.parse() {
                            Ok(a) => self.tm.white_time = a,
                            Err(_) => break,
                        },
                        None => break,
                    },

                    "btime" => match args.next() {
                        Some(arg) => match arg.parse() {
                            Ok(a) => self.tm.black_time = a,
                            Err(_) => break,
                        },
                        None => break,
                    },

                    "winc" => match args.next() {
                        Some(arg) => match arg.parse() {
                            Ok(a) => self.tm.white_inc = a,
                            Err(_) => break,
                        },
                        None => break,
                    },

                    "binc" => match args.next() {
                        Some(arg) => match arg.parse() {
                            Ok(a) => {
                                self.tm.black_inc = a
                            }
                            Err(_) => break,
                        },
                        None => break,
                    },

                    "movestogo" => match args.next() {
                        Some(arg) => match arg.parse() {
                            Ok(a) => self.tm.moves_to_go = a,
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
                        Some(arg) => match arg.parse::<u64>() {
                            Ok(a) => {
                                self.game.move_time = a * 9 / 10;
                                //self.timer_start();
                                //return;
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
        // timer not implemented
        //info!("Enter search with time {}", self.game.move_time);
        match self.game.find_move() {
            Some(m) => {
                let mut bresult = mem::MaybeUninit::<Board>::uninit();
                unsafe {
                    let _ = &self.game.board.make_move(m, &mut *bresult.as_mut_ptr());
                }
                let result = format!("bestmove {}", m.to_string());
                self.send_string(result.as_str());
            }
            None => error!("No valid move found"),
        }
    }

    fn uci(&self) {
        self.send_id();
        self.send_options();
        self.send_uci_ok();
    }

    fn send_id(&self) {
        self.send_string("id name C4E5R");
        self.send_string("id author Eugen Lindorfer");
    }

    fn send_options(&self) {
        self.send_string("option"); //TODO extend this
    }

    fn send_uci_ok(&self) {
        self.send_string("uciok");
    }

    fn send_ready_ok(&self) {
        self.send_string("readyok");
    }

    fn send_string(&self, s: &str) {
        println!("{}", s);
        info!("|   {}", s);
    }
}
