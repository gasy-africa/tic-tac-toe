```rust
use std::io;
use std::cmp::Ordering;

// Define the game board
type Board = [[char; 3]; 3];

// Define the players
#[derive(PartialEq, Clone, Copy)]
enum Player {
    Human,
    Computer,
}

// Define the game state
struct GameState {
    board: Board,
    current_player: Player,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            board: [[' '; 3]; 3],
            current_player: Player::Human,
        }
    }

    fn make_move(&mut self, row: usize, col: usize) {
        self.board[row][col] = match self.current_player {
            Player::Human => 'X',
            Player::Computer => 'O',
        };
    }

    fn get_empty_cells(&self) -> Vec<(usize, usize)> {
        let mut empty_cells = Vec::new();
        for row in 0..3 {
            for col in 0..3 {
                if self.board[row][col] == ' ' {
                    empty_cells.push((row, col));
                }
            }
        }
        empty_cells
    }

    fn is_terminal(&self) -> bool {
        // Check rows
        for row in 0..3 {
            if self.board[row][0] != ' ' && self.board[row][0] == self.board[row][1] && self.board[row][1] == self.board[row][2] {
                return true;
            }
        }

        // Check columns
        for col in 0..3 {
            if self.board[0][col] != ' ' && self.board[0][col] == self.board[1][col] && self.board[1][col] == self.board[2][col] {
                return true;
            }
        }

        // Check diagonals
        if self.board[0][0] != ' ' && self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] {
            return true;
        }
        if self.board[0][2] != ' ' && self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] {
            return true;
        }

        // Check if the board is full
        for row in 0..3 {
            for col in 0..3 {
                if self.board[row][col] == ' ' {
                    return false;
                }
            }
        }

        true
    }

    fn evaluate(&self) -> i32 {
        // Check rows
        for row in 0..3 {
            if self.board[row][0] != ' ' && self.board[row][0] == self.board[row][1] && self.board[row][1] == self.board[row][2] {
                return match self.board[row][0] {
                    'X' => -1,
                    'O' => 1,
                    _ => unreachable!(),
                };
            }
        }

        // Check columns
        for col in 0..3 {
            if self.board[0][col] != ' ' && self.board[0][col] == self.board[1][col] && self.board[1][col] == self.board[2][col] {
                return match self.board[0][col] {
                    'X' => -1,
                    'O' => 1,
                    _ => unreachable!(),
                };
            }
        }

        // Check diagonals
        if self.board[0][0] != ' ' && self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] {
            return match self.board[0][0] {
                'X' => -1,
                'O' => 1,
                _ => unreachable!(),
            };
        }
        if self.board[0][2] != ' ' && self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] {
            return match self.board[0][2] {
                'X' => -1,
                'O' => 1,
                _ => unreachable!(),
            };
        }

        0
    }

    fn print(&self) {
        for row in 0..3 {
            for col in 0..3 {
                print!(" {} ", self.board[row][col]);
                if col < 2 {
                    print!("|");
                }
            }
            println!();
            if row < 2 {
                println!("-----------");
            }
        }
        println!();
    }
}

// The Minimax algorithm
fn minimax(state: &mut GameState, depth: i32, maximizing_player: bool) -> i32 {
    if state.is_terminal() {
        return state.evaluate();
    }

    if maximizing_player {
        let mut max_eval = std::i32::MIN;

        let empty_cells = state.get_empty_cells();
        for &(row, col) in &empty_cells {
            state.make_move(row, col);
            let eval = minimax(state, depth - 1, false);
            state.make_move(row, col); // Undo the move
            max_eval = max_eval.max(eval);
        }

        max_eval
    } else {
        let mut min_eval = std::i32::MAX;

        let empty_cells = state.get_empty_cells();
        for &(row, col) in &empty_cells {
            state.make_move(row, col);
            let eval = minimax(state, depth - 1, true);
            state.make_move(row, col); // Undo the move
            min_eval = min_eval.min(eval);
        }

        min_eval
    }
}

// Function to get the best move for the computer player using Minimax
fn get_best_move(state: &mut GameState) -> (usize, usize) {
    let empty_cells = state.get_empty_cells();
    let mut best_move = None;
    let mut best_eval = std::i32::MIN;

    for &(row, col) in &empty_cells {
        state.make_move(row, col);
        let eval = minimax(state, 9, false);
        state.make_move(row, col); // Undo the move

        if eval > best_eval {
            best_eval = eval;
            best_move = Some((row, col));
        }
    }

    best_move.unwrap()
}

// Function to handle the human player's move
fn handle_human_move(state: &mut GameState) {
    loop {
        println!("Enter your move (row, col):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        let input: Vec<&str> = input.trim().split(',').collect();
        if input.len() == 2 {
            if let (Ok(row), Ok(col)) = (input[0].parse::<usize>(), input[1].parse::<usize>()) {
                if row < 3 && col < 3 && state.board[row][col] == ' ' {
                    state.make_move(row, col);
                    break;
                }
            }
        }

        println!("Invalid move. Please try again.");
    }
}

// Main game loop
fn main() {
    let mut state = GameState::new();

    println!("Welcome to Tic Tac Toe!");

    loop {
        state.print();

        match state.current_player {
            Player::Human => {
                handle_human_move(&mut state);
                if state.is_terminal() {
                    state.print();
                    match state.evaluate().cmp(&0) {
                        Ordering::Less => println!("You win!"),
                        Ordering::Greater => println!("Computer wins!"),
                        Ordering::Equal => println!("It's a draw!"),
                    }
                    break;
                }
                state.current_player = Player::Computer;
            }
            Player::Computer => {
                let (row, col) = get_best_move(&mut state);
                state.make_move(row, col);
                if state.is_terminal() {
                    state.print();
                    match state.evaluate().cmp(&0) {
                        Ordering::Less => println!("You win!"),
                        Ordering::Greater => println!("Computer wins!"),
                        Ordering::Equal => println!("It's a draw!"),
                    }
                    break;
                }
                state.current_player = Player::Human;
            }
        }
    }
}

```

## [:back:](../../#fourth-response)
