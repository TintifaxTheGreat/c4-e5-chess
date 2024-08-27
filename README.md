# C4-E5 Chess

[![Rust Build](https://github.com/TintifaxTheGreat/c4-e5-chess/actions/workflows/rust.yml/badge.svg)](https://github.com/TintifaxTheGreat/c4-e5-chess/actions/workflows/rust.yml)
[![Clippy Analyse](https://github.com/TintifaxTheGreat/c4-e5-chess/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/TintifaxTheGreat/c4-e5-chess/actions/workflows/rust-clippy.yml)

C4-E5 Chess is a [UCI](http://wbec-ridderkerk.nl/html/UCIProtocol.html) compatible chess engine based on the move generator in crate [Chess](https://docs.rs/chess/latest/chess/).

These features are provided:

- Iterative depthening (parallelised)
- Late move pruning
- Principal variant search
- Transposition table

At the time being, the focus was on simplicity and certainly there is a lot of potential in improvments in terms of playing strength.

## Documentation
https://docs.rs/c4-e5-chess/0.2.3

