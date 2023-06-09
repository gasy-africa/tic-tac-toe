# :abacus: Tic Tac Toe

---
**NOTE**

This project is a study of the [Minimax](https://en.wikipedia.org/wiki/Minimax) Algorithm, written in [Rust](https://www.rust-lang.org/), through game boards like the [Tic Tac Toe](https://en.wikipedia.org/wiki/Tic-tac-toe) or the [Fanorona](https://en.wikipedia.org/wiki/Fanorona). To facilitate the study, the project will generate and compare different source codes using a [Generative AI](https://en.wikipedia.org/wiki/Generative_artificial_intelligence) tools like [ChatGPT](https://chat.openai.com/)

The aproach is to :
1. generate a `Minimax Algorithm template`, 
2. generate several Tic-Tac-Toe :scroll: source code `examples` used as :newspaper_roll: source code `templates`
3. gather all the `Responses`
4. mix all the `reponses` by refactoringg the source code based on the first `Minimax Algorithm template`

**Summary**

Generate multiple source codes using Generative AI and refactor to the best source code based on the state of the art `Refactorization` rules.

---



| :hash: | Acivities |
|--------|-----------|
| [:a:](#a-algorithm-template)  | Success :heavy_check_mark:   |
| [:two:nd](#twond-response)  | TicTactoe :heavy_check_mark: |
| [:b:](#b-tictactoe)         | TicTactoe :heavy_check_mark: |
| [:ab:](#ab-mixing-solution) | Mixing :heavy_check_mark: |
| [:cl:](#cl-fanorona) | More Code | 

## :a: Algorithm Template 


```
generate minimax algorithm in rust
```
> Certainly! Here's an example implementation of the Minimax algorithm in Rust:

[:point_right: Click to view the generated source code :scroll: ](.docs/1/CODE.md)

Please note that this is a basic framework for the Minimax algorithm, and you'll need to replace the placeholders (...) with your actual game state type and implement the missing parts such as the evaluation function, generating possible moves, and checking for terminal game states.

Additionally, this implementation assumes a two-player, alternating-turn game. If you're working with a different game setup, you might need to modify the algorithm accordingly.

- [ ] Execute

```
cargo run
```
> Returns

[:point_right: Click to view the output when executing the source code :chart: ](.docs/1/OUTPUT.md)


## :two:nd Response


:recycle: `regenerate response`

[:point_right: Click to view the generated source code :scroll: ](.docs/2/CODE.md)

- [ ] Execute

```
cargo run
```
> Returns

[:point_right: Click to view the output when executing the source code :chart: ](.docs/2/OUTPUT.md)


## :b: TicTacToe

> generate tic tac toe game using minimax algorithm in rust

Sure! Here's an example implementation of Tic-Tac-Toe game using the Minimax algorithm in Rust:

[:point_right: Click to view the generated source code :scroll: ](.docs/3/CODE.md)

This code represents a simple command-line version of Tic-Tac-Toe where the player ('X') plays against the computer ('O') using the Minimax algorithm. The game prompts the player for their moves and displays the board after each move. The game continues until there is a winner or a draw.

Please note that this implementation does not include any advanced techniques like alpha-beta pruning, and it assumes that the player always enters valid moves. Feel free to enhance the code further according to your requirements.

- [ ] Execute

```
cargo run
```
> Returns :

[:point_right: Click to view the output when executing the source code :chart: ](.docs/3/OUTPUT.md)

## :ab: Mixing Solution

---
**NOTE**

The `mixing solution` is to take the `2nd` solution and add the `3rd` solution :keyboard: `input`, :desktop_computer: `output` and :loop: `loop` 

---


[:point_right: Click to view the generated source code :scroll: ](.docs/4/CODE.md)


- [ ] Execute

```
cargo run
```
> Returns :

[:point_right: Click to view the output when executing the source code :chart: ](.docs/4/OUTPUT.md)



## :cl: Fanorona

> generate a fanorona game written in rust

```rust
use std::io;

const BOARD_SIZE: usize = 5;

type Board = [[CellState; BOARD_SIZE]; BOARD_SIZE];

#[derive(Copy, Clone, PartialEq)]
enum CellState {
    Empty,
    White,
    Black,
}

struct Position {
    row: usize,
    col: usize,
}

struct Move {
    from: Position,
    to: Position,
}

struct Game {
    board: Board,
    current_player: CellState,
}

impl Game {
    fn new() -> Game {
        Game {
            board: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
            current_player: CellState::White,
        }
    }

    fn play(&mut self) {
        loop {
            self.display_board();

            let mv = self.get_move();
            if !self.is_valid_move(&mv) {
                println!("Invalid move! Please try again.");
                continue;
            }

            self.make_move(&mv);

            if self.is_game_over() {
                self.display_board();
                println!("Game over!");
                break;
            }

            self.switch_players();
        }
    }

    fn display_board(&self) {
        println!("Current board state:");
        for row in self.board.iter() {
            for cell in row.iter() {
                match cell {
                    CellState::Empty => print!("- "),
                    CellState::White => print!("W "),
                    CellState::Black => print!("B "),
                }
            }
            println!();
        }
    }

    fn get_move(&self) -> Move {
        println!("Player {:?}, enter your move (from_row from_col to_row to_col):", self.current_player);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input.");

        let positions: Vec<usize> = input.split_whitespace()
            .map(|s| s.parse().expect("Invalid input."))
            .collect();

        let from = Position { row: positions[0], col: positions[1] };
        let to = Position { row: positions[2], col: positions[3] };

        Move { from, to }
    }

    fn is_valid_move(&self, mv: &Move) -> bool {
        let from = mv.from;
        let to = mv.to;

        // Check if the 'from' and 'to' positions are within the board
        if from.row >= BOARD_SIZE || from.col >= BOARD_SIZE || to.row >= BOARD_SIZE || to.col >= BOARD_SIZE {
            return false;
        }

        // Check if the 'from' position is owned by the current player
        if self.board[from.row][from.col] != self.current_player {
            return false;
        }

        // Check if the 'to' position is empty
        if self.board[to.row][to.col] != CellState::Empty {
            return false;
        }

        // TODO: Implement more rules of Fanorona game for move validation

        true
    }

    fn make_move(&mut self, mv: &Move) {
        let from = mv.from;
        let to = mv.to;

        self.board[from.row][from.col] = CellState::Empty;
        self.board[to.row][to.col] = self.current_player;
    }

    fn is_game_over(&self) -> bool {
        // TODO: Implement game over condition for Fanorona

        false
    }

    fn switch_players(&mut self) {
        self.current_player = match self.current_player {
            CellState::White => CellState::Black,
            CellState::Black => CellState::White,
            _ => unreachable!(),
        };
    }
}

fn main() {
    let mut game = Game::new();
    game.play();
}

```
