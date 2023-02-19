use std::{io::stdin, str::SplitWhitespace};

pub struct Cli {}

impl Cli {
    pub fn execute(&self) {
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

    fn go(&self, mut args: SplitWhitespace) {
        //TODO Start time management

        loop {
            let option = args.next();
            match option {
                Some(_) => {}
                None => return,
            }
            let n = option.unwrap();

            match n {
                "searchmoves" => {
                    println!("SEARCH...")
                }
                _ => continue,
            }
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
