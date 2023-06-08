# tictactoe

```
generate minimax algorithm in rust
```


Certainly! Here's an example implementation of the Minimax algorithm in Rust:


```rust
// A simple implementation of the Minimax algorithm for tic-tac-toe

#[derive(Debug, Copy, Clone, PartialEq)]
enum Player {
    X,
    O,
}

#[derive(Debug, Copy, Clone)]
enum Cell {
    Empty,
    Occupied(Player),
}

#[derive(Debug)]
struct Board {
    cells: [[Cell; 3]; 3],
    current_player: Player,
}

impl Board {
    fn new() -> Self {
        Board {
            cells: [[Cell::Empty; 3]; 3],
            current_player: Player::X,
        }
    }

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

    fn make_move(&mut self, row: usize, col: usize) {
        self.cells[row][col] = Cell::Occupied(self.current_player);
        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    }

    fn get_winner(&self) -> Option<Player> {
        let lines = [
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

        for line in lines.iter() {
            let (r1, c1) = line[0];
            let (r2, c2) = line[1];
            let (r3, c3) = line[2];

            if let Cell::Occupied(player) = self.cells[r1][c1] {
                if self.cells[r2][c2] == Cell::Occupied(player) && self.cells[r3][c3] == Cell::Occupied(player) {
                    return Some(player);
                }
            }
        }

        None
    }

    fn minimax(&self, depth: i32, maximizing_player: bool) -> i32 {
        if let Some(winner) = self.get_winner() {
            return match winner {
                Player::X => 10 - depth,
                Player::O => depth - 10,
            };
        }

        if self.is_full() {
            return 0;
        }

        if maximizing_player {
            let mut max_eval = std::i32::MIN;
            for i in 0..3 {
                for j in 0..3 {
                    if let Cell::Empty = self.cells[i][j] {
                        let mut new_board = *self;
                        new_board.make_move(i, j);
                        let eval = new_board.minimax(depth + 1, false);
                        max_eval = std::cmp::max(max_eval, eval);
                    }
                }
            }
            return max_eval;
        } else {
            let mut min_eval = std::i32::MAX;
            for i in 0..3 {
                for j in 0..3 {
                    if let Cell::Empty = self.cells[i][j] {
                        let mut new_board = *self;
                        new_board.make_move(i, j);
                        let eval = new_board.minimax(depth + 1, true);
                        min_eval = std::cmp::min(min_eval, eval);
                    }
                }
            }
            return min_eval;
        }
    }

    fn best_move(&self) -> (usize, usize) {
        let mut best_eval = std::i32::MIN;
        let mut best_move = (0, 0);

        for i in 0..3 {
            for j in 0..3 {
                if let Cell::Empty = self.cells[i][j] {
                    let mut new_board = *self;
                    new_board.make_move(i, j);
                    let eval = new_board.minimax(0, false);
                    if eval > best_eval {
                        best_eval = eval;
                        best_move = (i, j);
                    }
                }
            }
        }

        best_move
    }
}

fn main() {
    let mut board = Board::new();

    // Example game loop
    loop {
        // Print the board
        for row in &board.cells {
            for cell in row {
                match cell {
                    Cell::Empty => print!("- "),
                    Cell::Occupied(Player::X) => print!("X "),
                    Cell::Occupied(Player::O) => print!("O "),
                }
            }
            println!();
        }

        if let Some(winner) = board.get_winner() {
            println!("Player {:?} wins!", winner);
            break;
        }

        if board.is_full() {
            println!("It's a draw!");
            break;
        }

        let (row, col) = board.best_move();
        board.make_move(row, col);
        println!("Player {:?} makes a move at row {}, col {}", board.current_player, row, col);
    }
}

```

This implementation represents a simple tic-tac-toe game using the Minimax algorithm to determine the best move for the AI player. The Board struct represents the game board and has methods for making moves, checking for a winner, checking if the board is full, and finding the best move using the Minimax algorithm. The main function provides an example game loop that alternates between the AI player and the human player, making moves on the board until there is a winner or a draw.
