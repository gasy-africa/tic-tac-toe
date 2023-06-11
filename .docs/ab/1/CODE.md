```rust
// A simple implementation of the Minimax algorithm for Tic Tac Toe

// Represents the players in the game
#[derive(Debug, Copy, Clone, PartialEq)]
enum Player {
    X,
    O,
}

// Represents the possible states of a Tic Tac Toe cell

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cell {
    Empty,
    X,
    O,
}

// Represents the Tic Tac Toe board
#[derive(Debug, Copy, Clone)]
struct Board {
    cells: [[Cell; 3]; 3],
}

impl Board {
    // Initializes a new empty board
    fn new() -> Self {
        Board {
            cells: [[Cell::Empty; 3]; 3],
        }
    }

    fn make_move(&mut self, row: usize, col: usize, player: Player) {
        self.cells[row][col] = match player {
            Player::X => Cell::X,
            Player::O => Cell::O,
        };
    }

    // Returns true if the board is full
    fn is_full(&self) -> bool {
        for row in &self.cells {
            for cell in row {
                if let Cell::Empty = cell {
                    return false;
                }
            }
        }
        true
    }

    // Returns true if the specified player has won
    fn has_won(&self, player: Player) -> bool {
        let symbol = match player {
            Player::X => Cell::X,
            Player::O => Cell::O,
        };

        // Check rows
        for row in &self.cells {
            if row.iter().all(|&cell| cell == symbol) {
                return true;
            }
        }

        // Check columns
        for col in 0..3 {
            if (0..3).all(|row| self.cells[row][col] == symbol) {
                return true;
            }
        }

        // Check diagonals
        if (0..3).all(|i| self.cells[i][i] == symbol)
            || (0..3).all(|i| self.cells[i][2 - i] == symbol)
        {
            return true;
        }

        false
    }

    // Evaluates the score of the current board state for the specified player
    fn evaluate(&self, player: Player) -> i32 {
        if self.has_won(player) {
            return 1;
        } else if self.has_won(match player {
            Player::X => Player::O,
            Player::O => Player::X,
        }) {
            return -1;
        }
        0
    }

    // Performs the Minimax algorithm to determine the best move for the specified player
    fn minimax(&self, depth: i32, maximizing_player: bool) -> i32 {
        if self.is_full() || self.has_won(Player::X) || self.has_won(Player::O) {
            self.evaluate(Player::O) // O is the AI player, X is the human player
        } else {
            let min_max = if maximizing_player {
                let mut max_eval = std::i32::MIN;
                for row in 0..3 {
                    for col in 0..3 {
                        if let Cell::Empty = self.cells[row][col] {
                            let mut new_board = self.clone();
                            new_board.cells[row][col] = Cell::O; // O is the AI player
                            let eval = new_board.minimax(depth + 1, false);
                            max_eval = max_eval.max(eval);
                        }
                    }
                }
                max_eval
            } else {
                let mut min_eval = std::i32::MAX;
                for row in 0..3 {
                    for col in 0..3 {
                        if let Cell::Empty = self.cells[row][col] {
                            let mut new_board = self.clone();
                            new_board.cells[row][col] = Cell::X; // X is the human player
                            let eval = new_board.minimax(depth + 1, true);
                            min_eval = min_eval.min(eval);
                        }
                    }
                }
                min_eval
            };
            min_max
        }
    }

    // Finds the best move for the AI player using the Minimax algorithm
    fn find_best_move(&self) -> (usize, usize) {
        let mut best_score = std::i32::MIN;
        let mut best_move = (0, 0);

        for row in 0..3 {
            for col in 0..3 {
                if let Cell::Empty = self.cells[row][col] {
                    let mut new_board = self.clone();
                    new_board.cells[row][col] = Cell::O; // O is the AI player
                    let score = new_board.minimax(0, false);
                    if score > best_score {
                        best_score = score;
                        best_move = (row, col);
                    }
                }
            }
        }

        best_move
    }
}

// Function to play the game
fn play_game() {
    let mut board: Board = Board::new();
    let mut current_player = Player::X;
    println!("Welcome to Tic-Tac-Toe!");

    loop {
        print_board(board);

        if board.evaluate(current_player) != 0 {
            match current_player {
                Player::X => println!("X wins!"),
                Player::O => println!("O wins!"),
            }
            break
        } else {
            if board.is_full() {
                println!("It's a draw!");
                break
            } else {
                match current_player {
                    Player::X => {
                        handle_human_move(&mut board, current_player);
                        current_player = Player::O
                    }
                    _ => {
                        // Find the best move for the AI player
                        let best_move: (usize,usize) = board.find_best_move();

                        println!("Computer plays: row={}, column={}", best_move.0, best_move.1);

                        board.make_move(best_move.0, best_move.1, current_player);
                        current_player = Player::X;
                    }
                }
            }
        }
    }
}

// Main function
fn main() {
    play_game();
}

// Function to print the board
fn print_board(board: Board) {
    for row in 0..3 {
        for col in 0..3 {
            let symbol = match board.cells[row][col] {
                Cell::O => "O",
                Cell::X => "X",
                _ => " ",
            };
            print!("| {}", symbol);
        }
        println!("|");
    }
}

use std::io;

// Function to handle the human player's move
fn handle_human_move(board: &mut Board, player: Player) {
    loop {
        println!("Enter your move (row, col):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        let input: Vec<&str> = input.trim().split(',').collect();
        if input.len() == 2 {
            if let (Ok(row), Ok(col)) = (input[0].parse::<usize>(), input[1].parse::<usize>()) {
                if row < 3 && col < 3 && board.cells[row][col] == Cell::Empty {
                    board.make_move(row, col, player);
                    break;
                }
            }
        }

        println!("Invalid move. Please try again.");
    }
}

```

## [:back:](../../../#ab-mixing-solution)
