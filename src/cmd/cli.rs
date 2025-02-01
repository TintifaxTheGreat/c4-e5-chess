use super::time_management::TimeManagement;
use crate::engine::game::Game;
use crate::misc::types::*;
use cozy_chess::{util, Board, Color};
use log::{error, info};
use std::{
    io::stdin,
    str::{FromStr, SplitWhitespace},
};

/// An UCI interface to be used with a chess GUI.
/// See https://en.wikipedia.org/wiki/Universal_Chess_Interface .
pub struct Cli {
    game: Game,
    tm: TimeManagement,
}

impl Cli {
    /// Constructor
    pub fn new() -> Cli {
        Cli {
            game: Default::default(),
            tm: TimeManagement::default(),
        }
    }

    /// Main execution loop.
    pub fn execute(&mut self) {
        loop {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let mut input_bak = input.clone();
            input_bak.pop();
            let input_bak_str = input_bak.as_str();
            let mut words = input.split_whitespace();

            match words.next() {
                Some(command) => {
                    let args = words;
                    info!("| {}", input_bak_str);
                    match command {
                        "uci" => {
                            self.send_id();
                            self.send_options();
                            self.send_uci_ok();
                        }

                        "isready" => {
                            self.send_ready_ok();
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
                None => continue,
            }
        }
    }

    /// UCI `position` command
    fn position(&mut self, mut args: SplitWhitespace) {
        while let Some(cmd) = args.next() {
            match cmd {
                "fen" => {
                    let mut fen: String = "".to_string();
                    for i in 0..6 {
                        match args.next() {
                            Some(s) => {
                                fen = fen + s + " ";
                                if i == 5 {
                                    // move count
                                    match s.parse::<MoveNumber>() {
                                        Ok(n) => self.game.move_number = n,
                                        Err(_) => error!("No move number in FEN"),
                                    }
                                }
                            }
                            None => {
                                error!("No FEN found");
                                return;
                            }
                        }
                    }
                    fen = fen.trim_end().to_string();
                    match Board::from_str(fen.as_str()) {
                        Ok(b) => self.game.board = b,
                        Err(e) => {
                            error!("FEN not valid: {}", e);
                            return;
                        }
                    }
                }

                // do nothing as game was already initialised with startposition
                "startpos" => {}

                "moves" => loop {
                    match args.next() {
                        Some(move_string) => {
                            match util::parse_uci_move(&self.game.board, move_string) {
                                Ok(m) => {
                                    info!("Move: {}", move_string);
                                    self.game.game_history.inc(&self.game.board);
                                    self.game.board.play_unchecked(m);
                                    if self.game.board.side_to_move() == Color::Black {
                                        self.game.move_number += 1;
                                    }
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
            }
        }
    }

    /// UCI `go` command
    fn go(&mut self, mut args: SplitWhitespace) {
        while let Some(cmd) = args.next() {
            match cmd {
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
                        Ok(a) => self.tm.black_inc = a,
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
                        }
                        Err(_) => break,
                    },
                    None => break,
                },

                _ => break,
            }
        }
        self.tm.set_game_time(&mut self.game);
        self.get_move_from_engine();
    }

    /// Get best move from the engine module.
    fn get_move_from_engine(&mut self) {
        match self.game.find_move() {
            Some(m) => {
                let result_uci = util::display_uci_move(&self.game.board, m);
                self.game.game_history.inc(&self.game.board);
                self.game.board.play_unchecked(m);
                let result = format!("bestmove {}", result_uci);
                info!("{} nodes examined.", self.game.node_count);
                self.send_string(result.as_str());
            }
            None => error!("No valid move found"),
        }
    }

    /// Send name and author.
    fn send_id(&self) {
        self.send_string("id name C4-E5 Chess");
        self.send_string("id author Eugen Lindorfer");
    }

    /// Send `options`.
    fn send_options(&self) {
        self.send_string("option"); //TODO extend this
    }

    /// Send `uci ok`.
    fn send_uci_ok(&self) {
        self.send_string("uciok");
    }

    /// Send `readyok`.
    fn send_ready_ok(&self) {
        self.send_string("readyok");
    }

    /// Output and log a string.
    fn send_string(&self, s: &str) {
        println!("{}", s);
        info!("|   {}", s);
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}
