use super::time_management::TimeManagement;
use crate::engine::game::Game;
use chess::Board;
use log::{error, info};
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
                        let fen: String = args.fold(String::new(), |acc, x| acc + x + " ");
                        // info!("FEN:|{}|", fen.clone());
                        match Board::from_str(fen.as_str()) {
                            Ok(b) => self.game.board = b,
                            Err(_) => {
                                error!("FEN not valid");
                            }
                        }
                        break;
                    }

                    // do nothing as game was already initialised with startposition
                    "startpos" => {}

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
                            Ok(a) => {info!("in binc");self.tm.black_inc = a}
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
        info!("im here");
        self.tm.set_game_time(&mut self.game);
        self.timer_start();
    }

    fn timer_start(&mut self) {
        let timer = Timer::new();
        let (tx, rx) = channel();

        self.game.playing.store(true, Ordering::Relaxed);
        let stop_bool = self.game.playing.clone();
        let _guard = timer.schedule_with_delay(
            chrono::Duration::milliseconds(self.game.move_time.clone() as i64),
            //chrono::Duration::seconds(10),
            move || {
                //info!("Game should stop NOW!!!!!");
                //stop_bool.store(false, Ordering::Relaxed);
                let _ignored = tx.send(());
                //info!("Game should stop NOW!!!!!");
            },
        );
        
        info!("Enter search with time {}", self.game.move_time);

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
        rx.recv().unwrap();
        info!("Now trying to stop the game");
        self.game.playing.store(false, Ordering::Relaxed);
        stop_bool.store(false, Ordering::Relaxed);

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
