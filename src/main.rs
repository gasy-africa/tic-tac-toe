// A simple implementation of the Minimax algorithm for Tic Tac Toe

// Represents the players in the game
#[derive(Debug, Copy, Clone, PartialEq)]
enum Player {
    X,
    O,
}

// Represents the possible states of a Tic Tac Toe cell

#[derive(Debug, Copy, Clone, PartialEq)]
enum Symbol {
    Empty,
    X,
    O,
}

// Represents the Tic Tac Toe board
#[derive(Debug, Copy, Clone)]
struct Board {
    cells: [[Symbol; 3]; 3],
}

impl Board {
    // Initializes a new empty board
    fn new() -> Self {
        Board {
            cells: [[Symbol::Empty; 3]; 3],
        }
    }

    // Returns true if the board is full
    fn is_full(&self) -> bool {
        for row in &self.cells {
            for cell in row {
                if let Symbol::Empty = cell {
                    return false;
                }
            }
        }
        true
    }

    // Returns true if the specified player has won
    fn has_won(&self, player: Player) -> bool {
        let symbol = match player {
            Player::X => Symbol::X,
            Player::O => Symbol::O,
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
                        if let Symbol::Empty = self.cells[row][col] {
                            let mut new_board = self.clone();
                            new_board.cells[row][col] = Symbol::O; // O is the AI player
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
                        if let Symbol::Empty = self.cells[row][col] {
                            let mut new_board = self.clone();
                            new_board.cells[row][col] = Symbol::X; // X is the human player
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

}

// Define the game state
struct GameState {
    board: Board,
    current_player: Player,
}

impl GameState {
    fn new() -> Self {
        Self {
            board: Board::new(),
            current_player: Player::X,
        }
    }

    fn make_move(&mut self, row: usize, col: usize) {
        self.board.cells[row][col] = match self.current_player {
            Player::X => Symbol::X,
            Player::O => Symbol::O,
        };
    }

    fn is_valid_move(&mut self, row: usize, col: usize) -> bool {
        if row < 3 && col < 3 && self.board.cells[row][col] == Symbol::Empty {
            true
        } else {
            false
        }
    }

    // Finds the best move for the AI player using the Minimax algorithm
    fn find_best_move(&self) -> (usize, usize) {
        let mut best_score = std::i32::MIN;
        let mut best_move = (0, 0);

        for row in 0..3 {
            for col in 0..3 {
                if let Symbol::Empty = self.board.cells[row][col] {
                    let mut new_board = self.board.clone();
                    new_board.cells[row][col] = Symbol::O; // O is the AI player
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

    fn is_terminal(&self) -> bool {

        match self.board.evaluate(self.current_player).cmp(&0) {
            Ordering::Equal => {
                if self.board.is_full() {
                    true
                } else {
                    false
                }
            }
            _ => true
        }

    }

    fn is_there_a_winner(&self) -> Symbol {
        match self.board.evaluate(self.current_player).cmp(&0) {
            Ordering::Greater =>
                Symbol::X,
            Ordering::Less =>
                Symbol::O,
            Ordering::Equal =>
                Symbol::Empty,
        }
    }

}

use std::cmp::Ordering;

// Function to play the game
fn play_game() {
    let mut game_state = GameState::new();
    println!("Welcome to Tic-Tac-Toe!");

    loop {
        print_board(game_state.board);

        if game_state.is_terminal() {
            print_results(game_state.is_there_a_winner()); 
            break
        } else {
            match game_state.current_player {
                Player::X => {
                    handle_human_move(&mut game_state);
                    game_state.current_player = Player::O
                }
                _ => {
                    // Find the best move for the AI player
                    let best_move: (usize,usize) = game_state.find_best_move();

                    println!("Computer plays: row={}, column={}", best_move.0, best_move.1);

                    game_state.make_move(best_move.0, best_move.1);
                    game_state.current_player = Player::X;
                }
            }
        }
    }
}

// Main function
fn main() {
    play_game();
}

fn print_results(symbol: Symbol) {
    match symbol {
        Symbol::X =>
            println!("X wins!"),
        Symbol::O =>
            println!("O wins!"),
        Symbol::Empty =>
            println!("It's a draw!"),
    }
}

// Function to print the board
fn print_board(board: Board) {
    for row in 0..3 {
        for col in 0..3 {
            let symbol = match board.cells[row][col] {
                Symbol::O => "O",
                Symbol::X => "X",
                _ => " ",
            };
            print!("| {}", symbol);
        }
        println!("|");
    }
}

use std::io;

// Function to handle the human player's move
fn handle_human_move(game_state: &mut GameState) {
    loop {
        println!("Enter your move (row, col):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        let input: Vec<&str> = input.trim().split(',').collect();
        if input.len() == 2 {
            if let (Ok(row), Ok(col)) = (input[0].parse::<usize>(), input[1].parse::<usize>()) {
                if game_state.is_valid_move(row, col) {
                    game_state.make_move(row, col);
                    break;
                }
            }
        }

        println!("Invalid move. Please try again.");
    }
}
