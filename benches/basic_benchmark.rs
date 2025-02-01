extern crate c4_e5_chess;

use std::str::FromStr;

use c4_e5_chess::{
    engine::{game::Game, move_gen::MoveGenPrime},
    eval::{evaluation::Evaluation, simple::Simple},
};
use cozy_chess::{Board, Move};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_evaluate(c: &mut Criterion) {
    let board = Board::default();
    c.bench_function("evaluate_simple", |b| {
        b.iter(|| Simple::evaluate(black_box(&board)))
    });
}

pub fn criterion_movegen(c: &mut Criterion) {
    let fen = "3q1rk1/4bp1p/1n2P2Q/1p1p1p2/6r1/Pp2R2N/1B1P2PP/7K w - - 0 1";
    let board = Board::from_fen(fen, false).unwrap();
    let mv = Move::from_str("h6h7").unwrap();
    c.bench_function("movegen_prime", |b| {
        b.iter(|| black_box(&board).get_legal_sorted(black_box(Some(mv))))
    });
}

pub fn criterion_find_move(c: &mut Criterion) {
    let fen = "4r1k1/5bpp/2p5/3pr3/8/1B3pPq/PPR2P2/2R2QK1 b - - 0 1";
    let mut game = Game::new(fen.to_string(), 0, 15000);
    c.bench_function("find_move", |b| b.iter(|| black_box(&mut game).find_move()));
}

criterion_group!(
    benches,
    criterion_evaluate,
    criterion_movegen,
    criterion_find_move
);
criterion_main!(benches);
