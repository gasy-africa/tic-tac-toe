```yaml
   Compiling tictactoe v0.1.0 (/Users/valiha/Developer/tictactoe)
warning: variable does not need to be mutable
   --> src/main.rs:158:9
    |
158 |     let mut best_move = new_board.find_best_move();
    |         ----^^^^^^^^^
    |         |
    |         help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` on by default

warning: `tictactoe` (bin "tictactoe") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.35s
     Running `target/debug/tictactoe`
Human player's move: row 1, column 1
AI player's best move: row 0, column 0
```

## [:back:](../#twond-response)
