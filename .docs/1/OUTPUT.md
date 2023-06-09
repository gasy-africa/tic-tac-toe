```yaml
   Compiling tictactoe v0.1.0 (/Users/valiha/Developer/tictactoe)
warning: unused variable: `player`
  --> src/main.rs:42:29
   |
42 |     fn make_move(&mut self, player: Player, position: usize) {
   |                             ^^^^^^ help: if this is intentional, prefix it with an underscore: `_player`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `position`
  --> src/main.rs:42:45
   |
42 |     fn make_move(&mut self, player: Player, position: usize) {
   |                                             ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_position`

warning: unused variable: `position`
  --> src/main.rs:49:29
   |
49 |     fn undo_move(&mut self, position: usize) {
   |                             ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_position`

warning: variable does not need to be mutable
  --> src/main.rs:38:13
   |
38 |         let mut vec = Vec::new();
   |             ----^^^
   |             |
   |             help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: field `board` is never read
  --> src/main.rs:11:5
   |
8  | struct GameState {
   |        --------- field in this struct
...
11 |     board: [Option<Player>; 9],
   |     ^^^^^
   |
   = note: `GameState` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` on by default

warning: `tictactoe` (bin "tictactoe") generated 5 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/tictactoe`
Best move: None
```

## [:back:](../../#a-algorithm-template)
