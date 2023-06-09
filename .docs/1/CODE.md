```rust
use std::io;

// Define the players and game symbols
#[derive(PartialEq)]
enum Player {
    Human,
    Computer,
}

#[derive(Clone, Copy, PartialEq)]
enum Symbol {
    Empty,
    X,
    O,
}

// Define the game state
struct GameState {
    board: [[Symbol; 3]; 3],
    current_player: Player,
}

impl GameState {
    fn new() -> Self {
        Self {
            board: [[Symbol::Empty; 3]; 3],
            current_player: Player::Human,
        }
    }

    fn is_valid_move(&self, row: usize, col: usize) -> bool {
        row < 3 && col < 3 && self.board[row][col] == Symbol::Empty
    }

    fn make_move(&mut self, row: usize, col: usize) {
        self.board[row][col] = match self.current_player {
            Player::Human => Symbol::X,
            Player::Computer => Symbol::O,
        };
        self.current_player = match self.current_player {
            Player::Human => Player::Computer,
            Player::Computer => Player::Human,
        };
    }

    fn get_winner(&self) -> Option<Player> {
        let winning_positions = [
            // Rows
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],
            // Columns
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],
            // Diagonals
            [(0, 0), (1, 1), (2, 2)],
            [(0, 2), (1, 1), (2, 0)],
        ];

        for positions in &winning_positions {
            let symbols: Vec<Symbol> = positions
                .iter()
                .map(|&(row, col)| self.board[row][col])
                .collect();

            if symbols == vec![Symbol::X, Symbol::X, Symbol::X] {
                return Some(Player::Human);
            } else if symbols == vec![Symbol::O, Symbol::O, Symbol::O] {
                return Some(Player::Computer);
            }
        }

        None
    }

    fn is_game_over(&self) -> bool {
        self.get_winner().is_some() || self.is_board_full()
    }

    fn is_board_full(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(|&symbol| symbol != Symbol::Empty))
    }

    fn get_available_moves(&self) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        for row in 0..3 {
            for col in 0..3 {
                if self.board[row][col] == Symbol::Empty {
                    moves.push((row, col));
                }
            }
        }
        moves
    }
}

// Define the Minimax algorithm
fn minimax(state: &mut GameState, depth: i32, maximizing_player: bool) -> i32 {
    if state.is_game_over() {
        match state.get_winner() {
            Some(Player::Human) => return -1,
            Some(Player::Computer) => return 1,
            None => return 0,
        }
    }

    if maximizing_player {
        let mut max_eval = std::i32::MIN;

        for (row, col) in state.get_available_moves() {
            if state.is_valid_move(row, col) {
                state.make_move(row, col);
                let eval = minimax(state, depth + 1, false);
                state.board[row][col] = Symbol::Empty;
                max_eval = max_eval.max(eval);
            }
        }

        return max_eval;
    } else {
        let mut min_eval = std::i32::MAX;

        for (row, col) in state.get_available_moves() {
            if state.is_valid_move(row, col) {
                state.make_move(row, col);
                let eval = minimax(state, depth + 1, true);
                state.board[row][col] = Symbol::Empty;
                min_eval = min_eval.min(eval);
            }
        }

        return min_eval;
    }
}

// Define the main game loop
fn play_tic_tac_toe() {
    let mut game_state = GameState::new();
    println!("Welcome to Tic Tac Toe!");

    loop {
        println!("{}", display_board(&game_state.board));

        if game_state.current_player == Player::Human {
            // Human player's turn
            println!("Your turn. Enter row (0-2):");
            let row = read_user_input();
            println!("Enter column (0-2):");
            let col = read_user_input();

            if game_state.is_valid_move(row, col) {
                game_state.make_move(row, col);
            } else {
                println!("Invalid move. Please try again.");
                continue;
            }
        } else {
            // Computer player's turn
            println!("Computer's turn...");

            let mut best_eval = std::i32::MIN;
            let mut best_move = (0, 0);

            for (row, col) in game_state.get_available_moves() {
                if game_state.is_valid_move(row, col) {
                    game_state.make_move(row, col);
                    let eval = minimax(&mut game_state, 0, false);
                    game_state.board[row][col] = Symbol::Empty;

                    if eval > best_eval {
                        best_eval = eval;
                        best_move = (row, col);
                    }
                }
            }

            game_state.make_move(best_move.0, best_move.1);
            println!("Computer placed a symbol.");
        }

        if let Some(winner) = game_state.get_winner() {
            println!("{}", display_board(&game_state.board));
            match winner {
                Player::Human => println!("You win!"),
                Player::Computer => println!("Computer wins!"),
            }
            break;
        }

        if game_state.is_board_full() {
            println!("{}", display_board(&game_state.board));
            println!("It's a tie!");
            break;
        }
    }
}

fn read_user_input() -> usize {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input.");

        match input.trim().parse::<usize>() {
            Ok(num) => {
                if num < 3 {
                    return num;
                } else {
                    println!("Invalid input. Please enter a number between 0 and 2.");
                }
            }
            Err(_) => {
                println!("Invalid input. Please enter a number between 0 and 2.");
            }
        }
    }
}

fn display_board(board: &[[Symbol; 3]; 3]) -> String {
    let symbols: Vec<Vec<char>> = board
        .iter()
        .map(|row| {
            row.iter()
                .map(|&symbol| match symbol {
                    Symbol::Empty => ' ',
                    Symbol::X => 'X',
                    Symbol::O => 'O',
                })
                .collect()
        })
        .collect();

    format!(
        "
         {} | {} | {}
        ---+---+---
         {} | {} | {}
        ---+---+---
         {} | {} | {}
        ",
        symbols[0][0],
        symbols[0][1],
        symbols[0][2],
        symbols[1][0],
        symbols[1][1],
        symbols[1][2],
        symbols[2][0],
        symbols[2][1],
        symbols[2][2]
    )
}

// Play the game
fn main() {
    play_tic_tac_toe();
}

```
