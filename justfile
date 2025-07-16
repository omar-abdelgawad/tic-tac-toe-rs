default:
  @just --list

against_comp:
  cargo run -r -- -c

two-player:
  cargo run -r 

test:
  @cargo test
