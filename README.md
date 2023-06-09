# tictactoe

```
generate minimax algorithm in rust
```


Certainly! Here's an example implementation of the Minimax algorithm in Rust:


| :hash: | Responses |
|--------|-----------|
| [:one:st](#onest-response)  | Success :heavy_check_mark: |
| [:two:nd](#twond-response)  | Success :heavy_check_mark: |
| [:b:](#b-tictactoe) | TicTactoe |

## :one:st Response 

```rust
#[derive(Debug, Copy, Clone, PartialEq)]
enum Player {
    Max,
    Min,
}

#[derive(Debug)]
struct GameState {
    // Define your game state representation here
    // Example:
    board: [Option<Player>; 9],
}

impl GameState {
    // Implement any necessary methods for your game state representation here
    // Example:
    fn is_terminal(&self) -> bool {
        // Check if the game is over
        // Example:
        // ...
        // Return true if the game is over, false otherwise
        false
    }

    fn evaluate(&self) -> i32 {
        // Evaluate the game state and return a score
        // Example:
        // ...
        // Return a score representing the utility of the game state
        0
    }

    fn get_possible_moves(&self) -> Vec<usize> {
        // Return a list of possible moves from the current game state
        // Example:
        // ...
        // Return a list of possible moves
        let mut vec = Vec::new();
        vec
    }

    fn make_move(&mut self, player: Player, position: usize) {
        // Update the game state with the given move
        // Example:
        // ...
        // Update the game state
    }

    fn undo_move(&mut self, position: usize) {
        // Undo the move made at the given position
        // Example:
        // ...
        // Undo the move
    }
}

fn minimax(
    state: &mut GameState,
    player: Player,
    depth: usize,
    mut alpha: i32,
    mut beta: i32,
) -> i32 {
    if state.is_terminal() || depth == 0 {
        return state.evaluate();
    }

    if player == Player::Max {
        let mut max_eval = std::i32::MIN;
        for position in state.get_possible_moves() {
            state.make_move(player, position);
            let eval = minimax(state, Player::Min, depth - 1, alpha, beta);
            state.undo_move(position);
            max_eval = max_eval.max(eval);
            alpha = alpha.max(eval);
            if beta <= alpha {
                break;
            }
        }
        return max_eval;
    } else {
        let mut min_eval = std::i32::MAX;
        for position in state.get_possible_moves() {
            state.make_move(player, position);
            let eval = minimax(state, Player::Max, depth - 1, alpha, beta);
            state.undo_move(position);
            min_eval = min_eval.min(eval);
            beta = beta.min(eval);
            if beta <= alpha {
                break;
            }
        }
        return min_eval;
    }
}

fn find_best_move(state: &mut GameState, player: Player, depth: usize) -> Option<usize> {
    let mut best_move = None;
    let mut best_eval = std::i32::MIN;

    for position in state.get_possible_moves() {
        state.make_move(player, position);
        let eval = minimax(state, Player::Min, depth - 1, std::i32::MIN, std::i32::MAX);
        state.undo_move(position);

        if eval > best_eval {
            best_eval = eval;
            best_move = Some(position);
        }
    }

    best_move
}

fn main() {
    let mut game_state = GameState {
        // Initialize your game state here
        // Example:
        board: [None; 9],
    };

    let best_move = find_best_move(&mut game_state, Player::Max, 4);

    println!("Best move: {:?}", best_move);
}

```

## :two:nd Response

```rust


// A simple implementation of the Minimax algorithm for Tic Tac Toe

// Represents the players in the game
#[derive(Debug, Copy, Clone)]
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

    // Returns the score of the current board state for the specified player
    fn score(&self, player: Player) -> i32 {
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
            self.score(Player::O) // O is the AI player, X is the human player
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

fn main() {
    let board = Board::new();

    // Make the human player's move
    // Assuming the human player chose the cell at row 1, column 1
    let mut new_board = board.clone();
    new_board.cells[1][1] = Cell::X; // X is the human player
    println!("Human player's move: row {}, column {}", 1, 1);

    // Find the best move for the AI player
    let mut best_move = new_board.find_best_move();

    println!("AI player's best move: row {}, column {}", best_move.0, best_move.1);

    new_board.cells[best_move.0][best_move.1] = Cell::O;

    new_board.cells[0][1] = Cell::X; // X is the human player
    println!("Human player's move: row {}, column {}", 0, 1);

    // Find the best move for the AI player
    best_move = new_board.find_best_move();

    println!("AI player's best move: row {}, column {}", best_move.0, best_move.1);

}


```

## :b: TicTacToe

> generate tic tac toe game using minimax algorithm in rust

```rust
use std::io;
use std::io::Write;

// Define the Tic-Tac-Toe board
type Board = [[Option<char>; 3]; 3];

// Function to print the board
fn print_board(board: &Board) {
    for row in board {
        for &cell in row {
            let symbol = match cell {
                Some(c) => c.to_string(),
                None => " ".to_string(),
            };
            print!("| {}", symbol);
        }
        println!("|");
    }
}

// Function to check if a player has won
fn check_win(board: &Board, player: char) -> bool {
    for i in 0..3 {
        // Check rows
        if board[i][0] == Some(player) && board[i][1] == Some(player) && board[i][2] == Some(player) {
            return true;
        }

        // Check columns
        if board[0][i] == Some(player) && board[1][i] == Some(player) && board[2][i] == Some(player) {
            return true;
        }
    }

    // Check diagonals
    if board[0][0] == Some(player) && board[1][1] == Some(player) && board[2][2] == Some(player) {
        return true;
    }
    if board[0][2] == Some(player) && board[1][1] == Some(player) && board[2][0] == Some(player) {
        return true;
    }

    false
}

// Function to check if the game is a draw
fn check_draw(board: &Board) -> bool {
    for row in board {
        for &cell in row {
            if cell.is_none() {
                return false;
            }
        }
    }
    true
}

// Function to get the available moves
fn get_available_moves(board: &Board) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            if board[i][j].is_none() {
                moves.push((i, j));
            }
        }
    }
    moves
}

// Minimax algorithm
fn minimax(board: &Board, depth: i32, maximizing_player: bool) -> i32 {
    if check_win(board, 'X') {
        return 1;
    }
    if check_win(board, 'O') {
        return -1;
    }
    if check_draw(board) {
        return 0;
    }

    if maximizing_player {
        let mut max_eval = std::i32::MIN;
        for (i, j) in get_available_moves(board) {
            let mut new_board = *board;
            new_board[i][j] = Some('X');
            let eval = minimax(&new_board, depth + 1, false);
            max_eval = max_eval.max(eval);
        }
        max_eval
    } else {
        let mut min_eval = std::i32::MAX;
        for (i, j) in get_available_moves(board) {
            let mut new_board = *board;
            new_board[i][j] = Some('O');
            let eval = minimax(&new_board, depth + 1, true);
            min_eval = min_eval.min(eval);
        }
        min_eval
    }
}

// Function to find the best move using Minimax
fn find_best_move(board: &Board) -> (usize, usize) {
    let mut best_move = (0, 0);
    let mut best_eval = std::i32::MIN;
    for (i, j) in get_available_moves(board) {
        let mut new_board = *board;
        new_board[i][j] = Some('X');
        let eval = minimax(&new_board, 0, false);
        if eval > best_eval {
            best_eval = eval;
            best_move = (i, j);
        }
    }
    best_move
}

// Function to play the game
fn play_game() {
    let mut board: Board = [[None; 3]; 3];
    let mut current_player = 'X';

    println!("Welcome to Tic-Tac-Toe!");

    loop {
        print_board(&board);

        if check_win(&board, 'X') {
            println!("X wins!");
            break;
        }

        if check_win(&board, 'O') {
            println!("O wins!");
            break;
        }

        if check_draw(&board) {
            println!("It's a draw!");
            break;
        }

        if current_player == 'X' {
            print!("Enter your move (row [0-2]): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let row: usize = input.trim().parse().unwrap();

            print!("Enter your move (column [0-2]): ");
            io::stdout().flush().unwrap();
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let column: usize = input.trim().parse().unwrap();

            if board[row][column].is_none() {
                board[row][column] = Some('X');
                current_player = 'O';
            } else {
                println!("Invalid move. Try again.");
            }
        } else {
            let (row, column) = find_best_move(&board);
            board[row][column] = Some('O');
            current_player = 'X';
            println!("Computer plays: row={}, column={}", row, column);
        }
    }
}

// Main function
fn main() {
    play_game();
}

```

- [ ] Execute

```
cargo run
```
> Returns :
```yaml
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/tictactoe`
Welcome to Tic-Tac-Toe!
|  |  |  |
|  |  |  |
|  |  |  |
Enter your move (row [0-2]): 1
Enter your move (column [0-2]): 1
|  |  |  |
|  | X|  |
|  |  |  |
Computer plays: row=0, column=0
| O|  |  |
|  | X|  |
|  |  |  |
Enter your move (row [0-2]): 0
Enter your move (column [0-2]): 1
| O| X|  |
|  | X|  |
|  |  |  |
Computer plays: row=0, column=2
| O| X| O|
|  | X|  |
|  |  |  |
Enter your move (row [0-2]): 2
Enter your move (column [0-2]): 1
| O| X| O|
|  | X|  |
|  | X|  |
X wins!
```


```
cargo run
```
> Returns :
```yaml
   Compiling tictactoe v0.1.0 (/Users/valiha/Developer/tictactoe)
    Finished dev [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/tictactoe`
Welcome to Tic-Tac-Toe!
|  |  |  |
|  |  |  |
|  |  |  |
Enter your move (row [0-2]): 0
Enter your move (column [0-2]): 0
| X|  |  |
|  |  |  |
|  |  |  |
Computer plays: row=1, column=1
| X|  |  |
|  | O|  |
|  |  |  |
Enter your move (row [0-2]): 0
Enter your move (column [0-2]): 1
| X| X|  |
|  | O|  |
|  |  |  |
Computer plays: row=0, column=2
| X| X| O|
|  | O|  |
|  |  |  |
Enter your move (row [0-2]): 2
Enter your move (column [0-2]): 0
| X| X| O|
|  | O|  |
| X|  |  |
Computer plays: row=1, column=0
| X| X| O|
| O| O|  |
| X|  |  |
Enter your move (row [0-2]): 1
Enter your move (column [0-2]): 2
| X| X| O|
| O| O| X|
| X|  |  |
Computer plays: row=2, column=1
| X| X| O|
| O| O| X|
| X| O|  |
Enter your move (row [0-2]): 2
Enter your move (column [0-2]): 2
| X| X| O|
| O| O| X|
| X| O| X|
It's a draw!
```
