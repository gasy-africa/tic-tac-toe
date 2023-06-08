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
