# chess
A simple chess engine. 
Because it's fun and because I needed an excuse to get accustomed to Rust.

## Status
- Move generation:
  - (done) all standard moves
  - (missing) check, checkmate, castling, promotion, en passant
- Check validity of human moves:
  - (done) all standard moves
  - (missing) check, checkmate, castling, promotion, en passant
- UI 
  - (in progress) make the engine [UCI](https://en.wikipedia.org/wiki/Universal_Chess_Interface)-compatible and use an available GUI
- Evaluations and heuristics:
  - (done) simple board value based on pieces
  - (done) simplified positional evaluation
  - (missing) more evaluations
- Search algorithm:
  - (done) min/max
  - (missing) quiescence search
- Game oprtimizations:
  - (done) alpha/beta pruning
  - (missing) move ordering
- Code optimizations:
  - (missing) more efficient board representations


